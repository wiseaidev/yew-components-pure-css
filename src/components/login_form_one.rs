use regex::Regex;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, Window};
use yew::prelude::*;

use crate::api::auth::login_user;
use input_yew::CustomInput;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct LoginUserSchema {
    email: String,
    password: String,
}

fn validate_email(email: String) -> bool {
    let pattern = Regex::new(r"^[^ ]+@[^ ]+\.[a-z]{2,3}$").unwrap();
    pattern.is_match(&email)
}

fn validate_password(password: String) -> bool {
    !&password.is_empty()
}

#[function_component(LoginFormOne)]
pub fn login_form_one() -> Html {
    let error_handle = use_state(String::default);
    let error = (*error_handle).clone();

    let email_valid_handle = use_state(|| true);
    let email_valid = (*email_valid_handle).clone();

    let password_valid_handle = use_state(|| true);
    let password_valid = (*password_valid_handle).clone();

    let input_email_ref = use_node_ref();
    let input_email_handle = use_state(String::default);
    let input_email = (*input_email_handle).clone();

    let input_password_ref = use_node_ref();
    let input_password_handle = use_state(String::default);
    let input_password = (*input_password_handle).clone();

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();

        let email_ref = input_password.clone();
        let password_ref = input_password.clone();
        let error_handle = error_handle.clone();
        console::log_1(&format!("Email: {}, Password: {}", input_email, input_password).into());

        spawn_local(async move {
            let email_val = email_ref.clone();
            let password_val = password_ref.clone();
            let error_handle = error_handle.clone();
            if email_valid && password_valid {
                let response = login_user(email_val, password_val).await;
                match response {
                    Ok(_) => {
                        console::log_1(&"success".into());
                        let window: Window = web_sys::window().expect("window not available");
                        let location = window.location();
                        let _ = location.set_href("/error");
                    }
                    Err(err) => {
                        error_handle.set(err);
                    }
                }
            } else {
                error_handle.set("Please provide a valid email and password!".into());
            }
        });
    });

    html! {
        <div class="form-one-content" role="main" aria-label="Sign In Form">
          <div class="text">
            <h2>{"Sign In"}</h2>
            if !error.is_empty() {
              <div class="error">{error}</div>
            }
          </div>
          <form action="#" aria-label="Sign In Form" onsubmit={onsubmit}>
              <CustomInput
                input_type={Some("text".to_string())}
                label={"".to_string()}
                input_handle={input_email_handle}
                name={"email".to_string()}
                input_ref={input_email_ref}
                input_placeholder={"Email".to_string()}
                icon_class={"fas fa-user".to_string()}
                error_message={"Enter a valid email address".to_string()}
                form_input_class={"".to_string()}
                form_input_field_class={"form-one-field".to_string()}
                form_input_label_class={"".to_string()}
                form_input_input_class={"".to_string()}
                form_input_error_class={"error-txt".to_string()}
                required={true}
                input_valid_handle={email_valid_handle}
                validate_function={validate_email}
              />
              <CustomInput
                input_type={Some("password".to_string())}
                label={"".to_string()}
                input_handle={input_password_handle}
                name={"password".to_string()}
                input_ref={input_password_ref}
                input_placeholder={"Password".to_string()}
                icon_class={"fas fa-lock".to_string()}
                error_message={"Password can't be blank!".to_string()}
                form_input_class={"".to_string()}
                form_input_field_class={"form-one-field".to_string()}
                form_input_label_class={"".to_string()}
                form_input_input_class={"".to_string()}
                form_input_error_class={"error-txt".to_string()}
                required={true}
                input_valid_handle={password_valid_handle}
                validate_function={validate_password}
                eye_active={"fa fa-eye"}
                eye_disabled={"fa fa-eye-slash"}
              />
            <div class="form-one-forgot-pass">
              <a href="#" aria-label="Forgot Password?">{"Forgot Password?"}</a>
            </div>
            <button type="submit">{"Sign in"}</button>
            <div class="sign-up">
              {"Not a member?"}
              <a href="#" aria-label="Sign up now">{"Sign up now"}</a>
            </div>
          </form>
          <div class="or-login-with">
            <h3>{"Or Log in With"}</h3>
          </div>
          <div class="social-buttons">
            <a
              class="social-button facebook-icon"
              href="#"
              target="_blank"
              rel="noopener noreferrer"
              aria-label="Facebook"
            >
              <i class="fab fa-facebook-f"></i>
            </a>
            <a
              class="social-button linkedin-icon"
              href="#"
              target="_blank"
              rel="noopener noreferrer"
              aria-label="LinkedIn"
            >
              <i class="fab fa-linkedin-in"></i>
            </a>
            <a
              class="social-button instagram-icon"
              href="#"
              target="_blank"
              rel="noopener noreferrer"
              aria-label="Instagram"
            >
              <i class="fab fa-instagram"></i>
            </a>
            <a
              class="social-button twitter-icon"
              href="#"
              target="_blank"
              rel="noopener noreferrer"
              aria-label="Twitter"
            >
              <i class="fab fa-twitter"></i>
            </a>
          </div>
        </div>
    }
}
