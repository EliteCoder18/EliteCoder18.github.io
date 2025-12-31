use leptos::prelude::*;

#[component]
pub fn Resume() -> impl IntoView {
    view! {
        <section style="max-width: 800px; margin: 40px auto; padding: 40px; background: #fff; border-radius: 8px; box-shadow: 0 4px 12px rgba(0,0,0,0.05);">
            <h2 style="text-align: center; color: #333; margin-bottom: 30px; font-size: 2rem;">"Resume"</h2>
            
            // Experience Section
            <div style="margin-bottom: 30px;">
                <h3 style="color: #d24827; border-bottom: 2px solid #eee; padding-bottom: 10px;">"Experience"</h3>
                
                <div style="margin-bottom: 20px;">
                    <h4 style="margin: 0 0 5px 0; font-size: 1.2rem;">"Software Developer"</h4>
                    <p style="font-style: italic; color: #666; margin: 0 0 10px 0;">"Company Name | 2024 - Present"</p>
                    <ul style="padding-left: 20px; color: #444; line-height: 1.6;">
                        <li>"Developed scalable web applications using Rust and WebAssembly."</li>
                        <li>"Collaborated with cross-functional teams to design and implement new features."</li>
                    </ul>
                </div>
            </div>

            // Education Section
            <div style="margin-bottom: 30px;">
                <h3 style="color: #d24827; border-bottom: 2px solid #eee; padding-bottom: 10px;">"Education"</h3>
                <div>
                    <h4 style="margin: 0 0 5px 0; font-size: 1.2rem;">"Bachelor of Technology in Computer Science"</h4>
                    <p style="font-style: italic; color: #666; margin: 0;">"University Name | 2021 - 2025"</p>
                </div>
            </div>

            // Skills Section
            <div style="margin-bottom: 40px;">
                <h3 style="color: #d24827; border-bottom: 2px solid #eee; padding-bottom: 10px;">"Skills"</h3>
                <div style="display: flex; flex-wrap: wrap; gap: 10px;">
                    <span style="background: #f4f4f9; padding: 6px 12px; border-radius: 4px; border: 1px solid #eee;">"Rust"</span>
                    <span style="background: #f4f4f9; padding: 6px 12px; border-radius: 4px; border: 1px solid #eee;">"Leptos"</span>
                    <span style="background: #f4f4f9; padding: 6px 12px; border-radius: 4px; border: 1px solid #eee;">"WebAssembly"</span>
                    <span style="background: #f4f4f9; padding: 6px 12px; border-radius: 4px; border: 1px solid #eee;">"HTML/CSS"</span>
                    <span style="background: #f4f4f9; padding: 6px 12px; border-radius: 4px; border: 1px solid #eee;">"Git"</span>
                </div>
            </div>
            
            // Download Button
            <div style="text-align: center;">
                <a href="#" // Replace with actual link to PDF
                   style="background: #333; color: white; padding: 12// filepath: /Users/rishitmodi/Desktop/Rishit_Coding/myportfolio_website/my-portfolio/src/components/resume.rs

                   px 24px; text-decoration: none; border-radius: 4px; font-weight: bold; transition: background 0.2s;">
"Download Full Resume"
</a>
</div>
</section>
}
}