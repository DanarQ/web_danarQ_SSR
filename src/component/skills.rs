use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn Skills() -> impl IntoView {
    view! {
        <div class="skills-container" id="skills">
            <h2>"Skills and Frameworks"</h2>
            <div class="skills-grid">
                <div class="skills-web-development">
                    <h3>"Web Development"</h3>
                    <ul>
                        <li>"Leptos" <Icon icon=icondata::SiLeptos/></li>
                        <li>"Next.js" <Icon icon=icondata::SiNextdotjs/></li>
                    </ul>
                </div>
                <div class="skills-mobile-development">
                    <h3>"Mobile Development"</h3>
                    <ul>
                        <li>"Flutter" <Icon icon=icondata::BiFlutter/></li>
                        <li>"React Native" <Icon icon=icondata::TbBrandReactNativeOutline/></li>
                    </ul>
                </div>
                <div class="skills-database">
                    <h3>"Database"</h3>
                    <ul>
                        <li>"Prisma" <Icon icon=icondata::SiPrisma/></li>
                        <li>"PostgresSQL" <Icon icon=icondata::BiPostgresql/></li>
                    </ul>
                </div>
                <div class="skills-programming-Languages">
                    <h3>"Programming Languages"</h3>
                    <ul>
                        <li>"Rust" <Icon icon=icondata::FaRustBrands/></li>
                        <li>"JavaScript" <Icon icon=icondata::BiJavascript/></li>
                        <li>"Python" <Icon icon=icondata::FaPythonBrands/></li>
                    </ul>
                </div>
            </div>
        </div>
    }
}
