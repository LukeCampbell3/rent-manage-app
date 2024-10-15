// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::{ComponentHandle, SharedString, ModelRc, VecModel};
use std::sync::{Arc, Mutex};

slint::include_modules!();

// Define the User struct with an array of properties
#[derive(Debug, Clone)]
struct User {
    email: String,
    password: String,
    property_owner: bool,
    properties: Vec<String>,  // An array of property locations
}

fn main() {
    // Initialize the app wrapped in Arc (Atomic Reference Counted)
    let app = Arc::new(App::new().expect("Failed to create app"));

    // State management for the user (using an Option to handle empty states initially)
    let user_state = Arc::new(Mutex::new(None::<User>));

    // Handle the 'create_account' callback
    let app_clone = Arc::clone(&app); // Clone Arc reference for use in the closure
    let user_state_clone = Arc::clone(&user_state);
    app.on_create_account(move || {
        let user_email = app_clone.get_user_email().to_string(); // Convert SharedString to String
        let user_password = app_clone.get_user_password().to_string(); // Convert SharedString to String
        let user_property_owner = app_clone.get_is_property_owner();


        // Check if the email and password are not empty
        if !user_email.is_empty() && !user_password.is_empty() {
            // Create a new User object, initializing an empty list of properties
            let new_user = User {
                email: user_email.clone(),
                password: user_password.clone(),
                property_owner: user_property_owner,
                properties: Vec::new(), // Initialize empty list of properties
            };

            // Store the new user in the user_state
            *user_state_clone.lock().unwrap() = Some(new_user);

            // Print user details (optional for debugging)
            println!(
                "User account created: email = {}, property owner = {}",
                user_email, user_property_owner
            );

            // Switch to the main state (state 2)
            app_clone.set_current_state(2);
        } else {
            // Handle the case where input fields are empty (optional error handling)
            println!("Error: Email and password must not be empty.");
        }
    });

    // Handle adding a property
    let app_clone = Arc::clone(&app);
    let user_state_clone = Arc::clone(&user_state);
    app.on_add_property(move |property_location| {
        let mut user_state = user_state_clone.lock().unwrap();
        if let Some(user) = user_state.as_mut() {
            // Add the new property location to the user's list of properties
            user.properties.push(property_location.to_string());

            // Convert Rust Vec<String> to Vec<SharedString> for Slint
            let slint_properties: Vec<SharedString> = user.properties.iter()
                .map(|p| SharedString::from(p.clone()))
                .collect();

            // Wrap Vec<SharedString> into ModelRc for Slint
            let model = ModelRc::new(VecModel::from(slint_properties));


            // Update the Slint properties array with the converted values
            app_clone.set_properties(model);
        }
    });

    // Run the app directly without unwrapping Arc
    app.run().unwrap();
}
