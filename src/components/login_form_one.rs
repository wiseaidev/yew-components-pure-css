use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlInputElement, Window};
use yew::prelude::*;

use crate::api::auth::login_user;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct LoginUserSchema {
    email: String,
    password: String,
}

#[function_component(LoginFormOne)]
pub fn login_form_one() -> Html {
    let error_handle = use_state(String::default);
    let error = (*error_handle).clone();

    let input_email_ref = use_node_ref();
    let input_email_handle = use_state(String::default);
    let input_email = (*input_email_handle).clone();

    let input_password_ref = use_node_ref();
    let input_password_handle = use_state(String::default);
    let input_password = (*input_password_handle).clone();

    let on_email_change = {
        let input_email_ref = input_email_ref.clone();

        Callback::from(move |_| {
            let input = input_email_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_email_handle.set(value);
            }
        })
    };

    let on_password_change = {
        let input_password_ref = input_password_ref.clone();

        Callback::from(move |_| {
            let input = input_password_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_password_handle.set(value);
            }
        })
    };

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

            <div class="form-one-field">
              <input
                id="email"
                type="text"
                name="username"
                placeholder="Email"
                aria-required="true"
                aria-label="Email"
                ref={input_email_ref}
                oninput={on_email_change}
              />
              <span
                class="fas fa-user"
                aria-hidden="true"
                role="presentation"
              ></span>
              <label for="email">{"Email or Phone"}</label>
            </div>
            <div class="form-one-field">
              <input
                id="password"
                type="password"
                name="pass"
                aria-required="true"
                aria-label="Password"
                placeholder="Password"
                ref={input_password_ref}
                oninput={on_password_change}
              />
              <span
                class="fas fa-lock"
                aria-hidden="true"
                role="presentation"
              ></span>
              <label for="password">{"Password"}</label>
            </div>
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
