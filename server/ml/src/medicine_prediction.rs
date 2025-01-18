use std::fs::remove_file;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};

use csv::ReaderBuilder;
use polars::prelude::*;
use rustlearn::metrics::accuracy_score;
use rustlearn::multiclass::OneVsRestWrapper;
use rustlearn::prelude::*;
use rustlearn::trees::decision_tree::{DecisionTree, Hyperparameters};

use serde_json::{json, Value};

pub fn read_csv_to_array(file_path: &str) -> (Array, Array) {
    /*
    This function takes input as the file path of csv file which holds our data
    for machine learning model and returns rustlearn type arrays of features and targets
     */

    // opening the CSV file
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => panic!("Error opening file {}: {}", file_path, e),
    };

    // creating a CSV reader
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    // reading CSV records and storing them in a vector of vectors
    let mut data: Vec<Vec<f32>> = Vec::new();
    for result in rdr.records() {
        let record = match result {
            Ok(record) => record,
            Err(e) => panic!("Error reading CSV record: {}", e),
        };
        let row: Vec<f32> = record
            .iter()
            .map(|s| s.parse::<f32>().expect("Error parsing integer"))
            .collect();
        data.push(row);
    }

    let mut features: Vec<Vec<f32>> = Vec::new();
    let mut target: Vec<f32> = Vec::new();

    for row in data {
        let row_len = row.len();

        // extracting features (all except last element)
        let row_features = row[..row_len - 1].to_vec();

        // extracting target (last element)
        let row_target = row[row_len - 1];

        features.push(row_features);
        target.push(row_target);
    }

    let features = Array::from(&features);
    let target = Array::from(target);

    (features, target)
}

pub fn get_medicine_data(generic: &str) -> Value {
    /*
    This function takes input as the predicted generic name of medicine, and outputs the
    Brand, Medicine, Contains, Manufacturer Name of respective medicines falling under the
    predicted generic name.
     */

    // path to csv file of medicine data
    let file_path = "statics/ml/MedicineData.csv";

    // reading the csv file into an dataframe
    let df = CsvReader::from_path(file_path)
        .expect("failed to read csv")
        .infer_schema(None) // infering schema from the CSV file
        .has_header(true) // getting headers from the csv file
        .finish()
        .expect("failed to create dataframe");

    let mut out = df
        .clone()
        .lazy()
        .filter(col("Generic").eq(lit(generic)))
        .collect()
        .expect("failed to get medicine data");

    // below is code for saving predicted data to csv format
    let temp_file_name = "predicted_medicine.csv";
    let mut data_file = std::fs::File::create(temp_file_name).unwrap();

    // saving data to temporary csv file
    CsvWriter::new(&mut data_file).finish(&mut out).unwrap();

    // creating json obj from csv file
    let json_obj = csv_to_json_obj(temp_file_name);

    // deleting the temp file
    remove_file(temp_file_name).unwrap();

    json_obj
}

pub fn read_elements_from_file(file_path: &str) -> Vec<String> {
    /*
    This function takes input as the file path of txt file which holds the names of our
    features (diseases) or targets (generic name) and returns and vector of strings that
    contains all the names of our features/target.
     */

    // opening the file
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error opening file '{}'", file_path);
            return Vec::new();
        }
    };

    // creating a vector to store the elements
    let mut elements = Vec::new();

    // creating a BufReader to efficiently read the file line by line
    let reader = io::BufReader::new(file);

    // iterating over each line in the file
    for line in reader.lines() {
        let element = match line {
            Ok(line) => line,
            Err(_) => {
                eprintln!("Error reading line from file '{}'", file_path);
                continue;
            }
        };
        // pushing the element into the vector
        elements.push(element);
    }

    // returning the vector containing all the elements from the text file
    elements
}

pub fn map_text_to_array(
    user_input: Vec<String>,
    feature_names: Vec<String>,
    num_features: usize,
) -> Array {
    /*
    This function takes the input from user (list of diseases) and converts it into
    an ruslearn array based on order of features (diseases)
     */

    let mut vec_zeros: Vec<f32> = vec![0.0; num_features];

    for item in user_input {
        if let Some(index) = feature_names.iter().position(|s| s == &item) {
            // println!("Index of '{}' found using position(): {}", item, index);
            vec_zeros[index] = 1.0;
        } else {
            println!("'{}' not found in the vector.", item);
        }
    }

    let mut array = Array::from(vec_zeros);
    array.reshape(1, num_features);

    array
}

pub fn save_medicine_model() {
    /*
    This function is for training the model on data, and saving it to .bin file,
    the function also gives the accuracy of trained model and prediction on sample data
     */

    // part 1 loading csv file into array

    let file_path = "statics/ml/medicine_full_num.csv";
    // getting features and target that are rustlearn arrays
    let (features, target) = read_csv_to_array(file_path);

    // part 2 creating vector of feature names and target names

    // feature names path
    let features_path = "statics/ml/diseases.txt";
    // reading feature names from the file
    let feature_names = read_elements_from_file(features_path);

    // reading target names
    let target_path = "statics/ml/generics.txt";
    let target_names = read_elements_from_file(target_path);

    // part 3 the machine learning model

    let num_features = feature_names.len();

    // decision tree model
    let mut model = Hyperparameters::new(num_features)
        .min_samples_split(5)
        .max_depth(40)
        .one_vs_rest();

    // training the model
    model.fit(&features, &target).unwrap();
    let y_pred = model.predict(&features).expect("failed to predict");

    // Serialize the model
    let serialized_model = bincode::serialize(&model).expect("Failed to serialize model.");

    // Save the serialized model to a file
    let mut file = File::create("model.bin").expect("Failed to create model file.");
    file.write_all(&serialized_model)
        .expect("Failed to write model to file.");

    println!("Model saving success");

    // calculating accuracy
    let accuracy = accuracy_score(&target, &y_pred);
    println!("Accuracy: {:.2}%", accuracy * 100.0);

    // predicting medicine based on user input

    // sample diseases
    let disease_1 = String::from("Asthma");
    let disease_2 = String::from("Allergic rhinitis");

    // vector containing all input diseases
    let user_input = vec![disease_1, disease_2];

    // converting user text input to array
    let array = map_text_to_array(user_input, feature_names, num_features);

    // getting prediction based on user input
    let prediction = model.predict(&array).expect("failed to predicct");

    // getting the f32 prediction output and converting it to usize
    let prediction = prediction.data().first().expect("failed to get prediction");
    let prediction: usize = *prediction as usize;

    // the predicted generic name
    let generic = target_names[prediction].clone();
    println!("Predicted Generic Name is {}", generic);

    // getting medicine data based on predited generic name
    get_medicine_data(generic.as_str());

    println!("Done")
}

pub fn get_medicine_prediction(user_input: Vec<String>) -> Value {
    /*
    This function takes the input of diseases from user, and outputs the predicted
    medicines, this functions takes feature, generic names from text statics/ml and loads
    saved model for prediction
     */

    // reading feature names from the file
    let features_path = "statics/ml/diseases.txt";
    let feature_names = read_elements_from_file(features_path);

    // reading target names  from the file
    let target_path = "statics/ml/generics.txt";
    let target_names = read_elements_from_file(target_path);

    let num_features = feature_names.len();

    // Loading the saved model from the binary file
    let mut file = File::open("statics/ml/model.bin").expect("Failed to open model file.");
    let mut serialized_model = Vec::new();
    file.read_to_end(&mut serialized_model)
        .expect("Failed to read model file.");

    // Deserializing the model
    let model: OneVsRestWrapper<DecisionTree> =
        bincode::deserialize(&serialized_model).expect("Failed to deserialize model.");

    println!("Model loaded successfully\n");

    // predicting medicine based on user input

    // converting user text input to array
    let array = map_text_to_array(user_input, feature_names, num_features);

    // getting prediction based on user input
    let prediction = model.predict(&array).expect("failed to predicct");

    // getting the f32 prediction output and converting it to usize
    let prediction = prediction.data().first().expect("failed to get prediction");
    let prediction: usize = *prediction as usize;

    // the predicted generic name
    let generic = target_names[prediction].clone();

    // getting medicine data based on predited generic name
    let json_obj = get_medicine_data(generic.as_str());

    json_obj
}

pub fn save_disease_model() {
    /*
    This function is for training the model on data, and saving it to .bin file,
    the function also gives the accuracy of trained model and prediction on sample data
     */

    // part 1 loading csv file into array

    let file_path = "statics/ml/symptoms_full_num.csv";
    // getting features and target that are rustlearn arrays
    let (features, target) = read_csv_to_array(file_path);

    // part 2 creating vector of feature names and target names

    // feature names path
    let features_path = "statics/ml/symptoms.txt";
    // reading feature names from the file
    let feature_names = read_elements_from_file(features_path);

    // reading target names
    let target_path = "statics/ml/diseases_symptoms.txt";
    let target_names = read_elements_from_file(target_path);

    // part 3 the machine learning model

    let num_features = feature_names.len();

    // decision tree model
    let mut model = Hyperparameters::new(num_features)
        .min_samples_split(5)
        .max_depth(40)
        .one_vs_rest();

    // training the model
    model.fit(&features, &target).unwrap();
    let y_pred = model.predict(&features).expect("failed to predict");

    // Serialize the model
    let serialized_model = bincode::serialize(&model).expect("Failed to serialize model.");

    // Save the serialized model to a file
    let mut file = File::create("symptoms_model.bin").expect("Failed to create model file.");
    file.write_all(&serialized_model)
        .expect("Failed to write model to file.");

    println!("Model saving success");

    // calculating accuracy
    let accuracy = accuracy_score(&target, &y_pred);
    println!("Accuracy: {:.2}%", accuracy * 100.0);

    // predicting disease based on symptoms input

    // sample symptoms
    let symptom_1 = String::from("Fatigue");
    let symptom_2 = String::from("Increased hunger");
    let symptom_3 = String::from("Slow healing of wounds");

    // vector containing all input diseases
    let user_input = vec![symptom_1, symptom_2, symptom_3];

    // converting user text input to array
    let array = map_text_to_array(user_input, feature_names, num_features);

    // getting prediction based on user input
    let prediction = model.predict(&array).expect("failed to predicct");

    // getting the f32 prediction output and converting it to usize
    let prediction = prediction.data().first().expect("failed to get prediction");
    let prediction: usize = *prediction as usize;

    // the predicted disease name
    let disease = target_names[prediction].clone();
    println!("Predicted Disease Name is {}", disease);

    println!("Done")
}

pub fn get_disease_prediction(user_input: Vec<String>) -> String {
    /*
    This function takes the input of symptoms from user, and outputs the predicted
    disease, this functions takes features, target diseases from text statics/ml and loads
    saved model for prediction
     */

    // reading feature names from the file
    let features_path = "statics/ml/symptoms.txt";
    let feature_names = read_elements_from_file(features_path);

    // reading target names  from the file
    let target_path = "statics/ml/diseases_symptoms.txt";
    let target_names = read_elements_from_file(target_path);

    let num_features = feature_names.len();

    // Loading the saved model from the binary file
    let mut file = File::open("statics/ml/symptoms_model.bin").expect("Failed to open model file.");
    let mut serialized_model = Vec::new();
    file.read_to_end(&mut serialized_model)
        .expect("Failed to read model file.");

    // Deserializing the model
    let model: OneVsRestWrapper<DecisionTree> =
        bincode::deserialize(&serialized_model).expect("Failed to deserialize model.");

    println!("Model loaded successfully\n");

    // predicting disease based on user input

    // converting user text input to array
    let array = map_text_to_array(user_input, feature_names, num_features);

    // getting prediction based on user input
    let prediction = model.predict(&array).expect("failed to predicct");

    // getting the f32 prediction output and converting it to usize
    let prediction = prediction.data().first().expect("failed to get prediction");
    let prediction: usize = *prediction as usize;

    // the predicted disease name

    target_names[prediction].clone()
}

fn csv_to_json_obj(file_name: &str) -> Value {
    // Opening the CSV file
    let file = File::open(file_name).unwrap();
    let mut rdr = ReaderBuilder::new().from_reader(file);

    // Reading the header row to get column names
    let headers = rdr.headers().unwrap().clone();

    // Reading CSV records and converting them into a JSON array
    let mut records = Vec::new();
    for result in rdr.records() {
        let record = result.unwrap();
        let mut json_obj = json!({});
        for (idx, field) in record.iter().enumerate() {
            let field_name = headers.get(idx).unwrap_or("unknown").to_string();
            json_obj[field_name] = Value::String(field.to_string());
        }
        records.push(json_obj);
    }

    // Converting the JSON array into a JSON object
    let json_obj = json!({ "data": records });

    json_obj
}
