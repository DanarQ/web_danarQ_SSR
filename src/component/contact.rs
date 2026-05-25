use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn Contact() -> impl IntoView {
    // Form fields
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (subject, set_subject) = signal(String::new());
    let (message, set_message) = signal(String::new());

    // State signals
    let (is_submitting, set_is_submitting) = signal(false);
    let (submit_success, set_submit_success) = signal(false);
    let (error_message, set_error_message) = signal(Option::<String>::None);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let n = name.get().trim().to_string();
        let e = email.get().trim().to_string();
        let s = subject.get().trim().to_string();
        let m = message.get().trim().to_string();

        if n.is_empty() || e.is_empty() || s.is_empty() || m.is_empty() {
            set_error_message.set(Some("Please fill in all fields.".to_string()));
            return;
        }

        if !e.contains('@') || !e.contains('.') {
            set_error_message.set(Some("Please enter a valid email address.".to_string()));
            return;
        }

        set_error_message.set(None);
        set_is_submitting.set(true);

        #[cfg(target_arch = "wasm32")]
        leptos::task::spawn_local(async move {
            let response = submit_contact_form(n, e, s, m).await;

            match response {
                Ok(_) => {
                    set_is_submitting.set(false);
                    set_submit_success.set(true);
                    set_name.set(String::new());
                    set_email.set(String::new());
                    set_subject.set(String::new());
                    set_message.set(String::new());
                }
                Err(err) => {
                    set_is_submitting.set(false);
                    set_error_message.set(Some(format!(
                        "Failed to send message: {}",
                        err
                    )));
                }
            }
        });

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (n, e, s, m, set_is_submitting, set_submit_success, set_error_message, set_name, set_email, set_subject, set_message);
        }
    };

    view! {
        <section class="contact-section" id="contact">
            <div class="contact-container">
                <div class="contact-header">
                    <h2>"Get In Touch"</h2>
                    <p>"Have a question, a project idea, or just want to say hello? Drop me a message!"</p>
                </div>

                <div class="contact-grid">
                    // Left Column: Contact info & socials
                    <div class="contact-info">
                        <h3>"Contact Information"</h3>
                        <p class="contact-info-subtitle">"Let's build something amazing together! You can also reach me directly through my socials."</p>

                        <div class="contact-cards">
                            <div class="contact-card">
                                <div class="contact-card-icon">
                                    <Icon icon=icondata::FiMail/>
                                </div>
                                <div class="contact-card-content">
                                    <h4>"Email"</h4>
                                    <a href="mailto:danarqusyairi03@gmail.com">"danarqusyairi03@gmail.com"</a>
                                </div>
                            </div>

                            <div class="contact-card">
                                <div class="contact-card-icon">
                                    <Icon icon=icondata::FiMapPin/>
                                </div>
                                <div class="contact-card-content">
                                    <h4>"Location"</h4>
                                    <span>"Pontianak, Kalimantan Barat, Indonesia"</span>
                                </div>
                            </div>
                        </div>

                        <div class="contact-socials">
                            <h4>"Connect with me"</h4>
                            <div class="contact-social-links">
                                <a href="https://github.com/DanarQ" target="_blank" rel="noopener noreferrer" class="social-link-btn github">
                                    <Icon icon=icondata::FaGithubBrands/>
                                    <span>"GitHub"</span>
                                </a>
                                <a href="https://linkedin.com/in/danar-qusyairi" target="_blank" rel="noopener noreferrer" class="social-link-btn linkedin">
                                    <Icon icon=icondata::FaLinkedinBrands/>
                                    <span>"LinkedIn"</span>
                                </a>
                            </div>
                        </div>
                    </div>

                    // Right Column: Form or Success State
                    <div class="contact-form-wrapper">
                        {move || {
                            if submit_success.get() {
                                view! {
                                    <div class="contact-success-card">
                                        <div class="success-icon-wrapper">
                                            <Icon icon=icondata::FiCheckCircle/>
                                        </div>
                                        <h3>"Message Sent!"</h3>
                                        <p>"Thank you for reaching out. I have received your message and will get back to you as soon as possible."</p>
                                        <button class="btn-reset" on:click=move |_| set_submit_success.set(false)>
                                            "Send another message"
                                        </button>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <form on:submit=on_submit class="contact-form">
                                        // Error message
                                        {move || {
                                            error_message.get().map(|msg| {
                                                view! {
                                                    <div class="contact-error-notice">
                                                        <Icon icon=icondata::FiAlertCircle/>
                                                        <span>{msg}</span>
                                                    </div>
                                                }
                                            })
                                        }}

                                        <div class="form-row">
                                            <div class="form-group">
                                                <label for="name">"Your Name"</label>
                                                <input
                                                    type="text"
                                                    id="name"
                                                    placeholder="John Doe"
                                                    disabled=is_submitting
                                                    prop:value=name
                                                    on:input:target=move |ev| set_name.set(ev.target().value())
                                                />
                                            </div>
                                            <div class="form-group">
                                                <label for="email">"Email Address"</label>
                                                <input
                                                    type="email"
                                                    id="email"
                                                    placeholder="john@example.com"
                                                    disabled=is_submitting
                                                    prop:value=email
                                                    on:input:target=move |ev| set_email.set(ev.target().value())
                                                />
                                            </div>
                                        </div>

                                        <div class="form-group">
                                            <label for="subject">"Subject"</label>
                                            <input
                                                type="text"
                                                id="subject"
                                                placeholder="Project discussion..."
                                                disabled=is_submitting
                                                prop:value=subject
                                                on:input:target=move |ev| set_subject.set(ev.target().value())
                                            />
                                        </div>

                                        <div class="form-group">
                                            <label for="message">"Your Message"</label>
                                            <textarea
                                                id="message"
                                                rows="6"
                                                placeholder="Tell me about your project..."
                                                disabled=is_submitting
                                                prop:value=message
                                                on:input:target=move |ev| set_message.set(ev.target().value())
                                            ></textarea>
                                        </div>

                                        <button type="submit" class="submit-btn" disabled=is_submitting>
                                            {move || {
                                                if is_submitting.get() {
                                                    view! {
                                                        <div class="btn-spinner-wrapper">
                                                            <div class="spinner"></div>
                                                            <span>"Sending..."</span>
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <div class="btn-content">
                                                            <span>"Send Message"</span>
                                                            <Icon icon=icondata::FiSend/>
                                                        </div>
                                                    }.into_any()
                                                }
                                            }}
                                        </button>
                                    </form>
                                }.into_any()
                            }
                        }}
                    </div>
                </div>
            </div>
        </section>
    }
}

#[server(SubmitContactForm, "/api")]
pub async fn submit_contact_form(
    name: String,
    email: String,
    subject: String,
    message: String,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let key = std::env::var("WEB3FROM_ACCESS_KEY").unwrap_or_default();
        if key.is_empty() {
            return Err(ServerFnError::new("WEB3FROM_ACCESS_KEY is not set"));
        }

        #[derive(serde::Serialize)]
        struct Web3FormsPayload {
            access_key: String,
            name: String,
            email: String,
            subject: String,
            message: String,
        }

        let payload = Web3FormsPayload {
            access_key: key,
            name,
            email,
            subject,
            message,
        };

        let client = reqwest::Client::new();
        let response = client
            .post("https://api.web3forms.com/submit")
            .json(&payload)
            .send()
            .await
            .map_err(|e| ServerFnError::new(format!("Network error: {}", e)))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(ServerFnError::new(format!("API returned status: {}", response.status())))
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (name, email, subject, message);
        unreachable!()
    }
}
