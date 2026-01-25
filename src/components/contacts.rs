use leptos::*;

#[component]
pub fn Contacts() -> impl IntoView {
    view! {
        <div class="contacts-container">
            <div class="contacts-list">
                <a href="mailto:contact@skharchikov.com" target="_blank" rel="noopener noreferrer" class="contact-link-wrapper">
                    <div class="contact-button">
                        <div class="contact-button-inner">
                            <span class="contact-icon">
                                <svg stroke="currentColor" fill="none" stroke-width="2" viewBox="0 0 24 24" stroke-linecap="round" stroke-linejoin="round" height="22" width="22" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path>
                                    <polyline points="22,6 12,13 2,6"></polyline>
                                </svg>
                            </span>
                            <div class="contact-text">
                                <p>"contact@skharchikov.com"</p>
                            </div>
                        </div>
                    </div>
                </a>

                <a href="https://github.com/skharchikov" target="_blank" rel="noopener noreferrer" class="contact-link-wrapper">
                    <div class="contact-button">
                        <div class="contact-button-inner">
                            <span class="contact-icon">
                                <svg stroke="currentColor" fill="none" stroke-width="2" viewBox="0 0 24 24" stroke-linecap="round" stroke-linejoin="round" height="22" width="22" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
                                </svg>
                            </span>
                            <div class="contact-text">
                                <p>"skharchikov"</p>
                            </div>
                        </div>
                    </div>
                </a>

                <a href="https://linkedin.com/in/skharchikov" target="_blank" rel="noopener noreferrer" class="contact-link-wrapper">
                    <div class="contact-button">
                        <div class="contact-button-inner">
                            <span class="contact-icon">
                                <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 24 24" height="22" width="22" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z"></path>
                                </svg>
                            </span>
                            <div class="contact-text">
                                <p>"skharchikov"</p>
                            </div>
                        </div>
                    </div>
                </a>
            </div>
        </div>
    }
}
