use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt;
use std::str::FromStr;
use std::any::Any;
use serde_json::Value;

// Shared state structure with mutable counter
struct SharedState {
    counter: usize,
    // Add more shared state as needed
}

// Enum to represent different types of complex data
enum ComplexData {
    Integer(i32, Rc<RefCell<SharedState>>),
    Float(f64, Rc<RefCell<SharedState>>),
    Text(String, Rc<RefCell<SharedState>>),
    SliceOfStrings(Vec<String>, Rc<RefCell<SharedState>>), // New variant for a slice of strings
    Array(Vec<i32>, Rc<RefCell<SharedState>>),
    Object(Value, Rc<RefCell<SharedState>>),
    ArrayOfObjects(Vec<Value>, Rc<RefCell<SharedState>>),
}

impl ComplexData {
    // Increment the shared counter within the shared state
    fn increment_shared_counter(&self) {
        match self {
            ComplexData::Integer(_, shared_state) |
            ComplexData::Float(_, shared_state) |
            ComplexData::Text(_, shared_state) |
            ComplexData::SliceOfStrings(_, shared_state) |
            ComplexData::Array(_, shared_state) |
            ComplexData::Object(_, shared_state) |
            ComplexData::ArrayOfObjects(_, shared_state) => {
                shared_state.borrow_mut().counter += 1;
                println!("Shared Counter: {}", shared_state.borrow().counter);
            }
        }
    }
}

impl fmt::Display for ComplexData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComplexData::Integer(val, _) => write!(f, "Integer: {}", val),
            ComplexData::Float(val, _) => write!(f, "Float: {}", val),
            ComplexData::Text(val, _) => write!(f, "Text: {}", val),
            ComplexData::SliceOfStrings(strings, _) => write!(f, "Slice of Strings: {:?}", strings),
            ComplexData::Array(arr, _) => write!(f, "Array: {:?}", arr),
            ComplexData::Object(obj, _) => write!(f, "Object: {:?}", obj),
            ComplexData::ArrayOfObjects(arr, _) => write!(f, "Array of Objects: {:?}", arr),
        }
    }
}impl FromStr for ComplexData {
    type Err = ComplexDataParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Implement parsing logic here
        // For simplicity, let's assume the input string is in the format "Integer: 42"
        if s.starts_with("Integer: ") {
            let val: i32 = s.trim_start_matches("Integer: ").parse().map_err(|_| ComplexDataParseError::InvalidInteger)?;
            Ok(ComplexData::Integer(val, Rc::new(RefCell::new(SharedState { counter: 0 }))))
        } else if s.starts_with("Float: ") {
            let val: f64 = s.trim_start_matches("Float: ").parse().map_err(|_| ComplexDataParseError::InvalidFloat)?;
            Ok(ComplexData::Float(val, Rc::new(RefCell::new(SharedState { counter: 0 }))))
        } else if s.starts_with("Text: ") {
            let val: String = s.trim_start_matches("Text: ").to_string();
            Ok(ComplexData::Text(val, Rc::new(RefCell::new(SharedState { counter: 0 }))))
        } else if s.starts_with("Slice of Strings: ") {
            // Assume the input string is in the format "Slice of Strings: ["one", "two", "three"]"
            let strings_str = s.trim_start_matches("Slice of Strings: ");
            let strings: Vec<String> = serde_json::from_str(strings_str).map_err(|_| ComplexDataParseError::InvalidSliceOfStrings)?;
            Ok(ComplexData::SliceOfStrings(strings, Rc::new(RefCell::new(SharedState { counter: 0 }))))
        } else if s.starts_with("Array: ") {
            // Assume the input string is in the format "Array: [1, 2, 3]"
            let arr_str = s.trim_start_matches("Array: ");
            let arr: Vec<i32> = serde_json::from_str(arr_str).map_err(|_| ComplexDataParseError::InvalidArray)?;
            Ok(ComplexData::Array(arr, Rc::new(RefCell::new(SharedState { counter: 0 }))))
        } else if s.starts_with("Object: ") {
            // Assume the input string is in the format "Object: {"key": "value"}"
            let obj_str = s.trim_start_matches("Object: ");
            let obj: Value = serde_json::from_str(obj_str).map_err(|_| ComplexDataParseError::InvalidObject)?;
            Ok(ComplexData::Object(obj, Rc::new(RefCell::new(SharedState { counter: 0 }))))
        } else if s.starts_with("Array of Objects: ") {
            // Assume the input string is in the format "Array of Objects: [{"name": "Alice"}, {"name": "Bob"}]"
            let arr_obj_str = s.trim_start_matches("Array of Objects: ");
            let arr_obj: Vec<Value> = serde_json::from_str(arr_obj_str).map_err(|_| ComplexDataParseError::InvalidArrayOfObjects)?;
            Ok(ComplexData::ArrayOfObjects(arr_obj, Rc::new(RefCell::new(SharedState { counter: 0 }))))
        } else {
            Err(ComplexDataParseError::InvalidFormat)
        }
    }
}

#[derive(Debug)]
enum ComplexDataParseError {
    InvalidInteger,
    InvalidFloat,
    InvalidText,
    InvalidSliceOfStrings,
    InvalidArray,
    InvalidObject,
    InvalidArrayOfObjects,
    InvalidFormat,
}


impl fmt::Display for ComplexDataParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComplexDataParseError::InvalidInteger => write!(f, "Invalid Integer format"),
            ComplexDataParseError::InvalidFloat => write!(f, "Invalid Float format"),
            ComplexDataParseError::InvalidText => write!(f, "Invalid Text format"),
            ComplexDataParseError::InvalidSliceOfStrings => write!(f, "Invalid Slice of Strings format"),
            ComplexDataParseError::InvalidArray => write!(f, "Invalid Array format"),
            ComplexDataParseError::InvalidObject => write!(f, "Invalid Object format"),
            ComplexDataParseError::InvalidArrayOfObjects => write!(f, "Invalid Array of Objects format"),
            ComplexDataParseError::InvalidFormat => write!(f, "Invalid format"),
        }
    }
}


// Trait for processing data
trait DataProcessor {
    fn process(&self);
    fn to_string(&self) -> String;
    fn increment_shared_counter(&self);
    fn as_any(&self) -> &(dyn Any + 'static); // Changed the lifetime to 'static
}

impl DataProcessor for ComplexData {
    fn process(&self) {
        println!("{}", self);
    }

    fn to_string(&self) -> String {
        format!("{}", self)
    }

    fn increment_shared_counter(&self) {
        ComplexData::increment_shared_counter(self);
    }

    fn as_any(&self) -> &(dyn Any + 'static) {
        self
    }
}

impl DataProcessor for String {
    fn process(&self) {
        println!("Processing String: {}", self);
    }

    fn to_string(&self) -> String {
        self.clone()
    }

    fn increment_shared_counter(&self) {
        // Handle increment logic for String if needed
    }

    fn as_any(&self) -> &(dyn Any + 'static) {
        self
    }
}

// Container struct to hold data with trait object
struct Container {
    data: Rc<dyn DataProcessor>,
}

impl Container {
    fn new(data: Rc<dyn DataProcessor>) -> Self {
        Container { data }
    }

    fn process_data(&self) {
        self.data.process();
    }

    fn print_data(&self) {
        println!("{}", self.data.to_string());
    }

    fn increment_shared_counter(&self) {
        if let Some(complex_data) = self.data.as_any().downcast_ref::<ComplexData>() {
            complex_data.increment_shared_counter();
        }
    }
}
fn main() {
    // Example data of different types
    let int_data = "Integer: 42";
    let float_data = "Float: 3.14";
    let text_data = "Text: Hello";
    let slice_of_strings_data = "Slice of Strings: [\"one\", \"two\", \"three\"]";
    let array_data = "Array: [1, 2, 3]";
    let object_data = "Object: {\"key\": \"value\"}";
    let array_of_objects_data = "Array of Objects: [{\"name\": \"Alice\"}, {\"name\": \"Bob\"}]";

    // Convert different data types into ComplexData
    let int_complex_data = ComplexData::from_str(int_data);
    let float_complex_data = ComplexData::from_str(float_data);
    let text_complex_data = ComplexData::from_str(text_data);
    let slice_of_strings_complex_data = ComplexData::from_str(slice_of_strings_data);
    let array_complex_data = ComplexData::from_str(array_data);
    let object_complex_data = ComplexData::from_str(object_data);
    let array_of_objects_complex_data = ComplexData::from_str(array_of_objects_data);

    // Handle the results
    handle_complex_data(int_complex_data);
    handle_complex_data(float_complex_data);
    handle_complex_data(text_complex_data);
    handle_complex_data(slice_of_strings_complex_data);
    handle_complex_data(array_complex_data);
    handle_complex_data(object_complex_data);
    handle_complex_data(array_of_objects_complex_data);
}

// Function to handle Result<ComplexData, ComplexDataParseError>
fn handle_complex_data(result: Result<ComplexData, ComplexDataParseError>) {
    match result {
        Ok(data) => {
            let container = Container::new(Rc::new(data));
            container.process_data();
            container.print_data();
            container.increment_shared_counter();
        }
        Err(err) => println!("Error: {}", err),
    }
}