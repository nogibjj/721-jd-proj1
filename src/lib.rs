// read in h2h data
// read in model training data 
// train model 
// dataframe of predictions? 

// function to find h2h 
// function to find results 

pub fn read_csv(filename: &str) -> Vec<f64> {
    let mut rdr = csv::Reader::from_path(filename).unwrap();
    let mut data: Vec<f64> = Vec::new();
    for result in rdr.records() {
        let record = result.unwrap();
        let value: f64 = record[0].parse().unwrap();
        data.push(value);
    }
    data
}


