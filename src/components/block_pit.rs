use leptos::prelude::*;
use leptos::ev::MouseEvent;
use leptos::html::Div;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use std::rc::Rc;
use std::cell::RefCell;
use leptos::leptos_dom::helpers::request_animation_frame;

#[derive(Clone, Copy, Debug)]
struct PhysicsObject {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    width: f64,
    height: f64,
    rotation: f64,
    vr: f64,
    is_sleeping: bool,
}

#[component]
pub fn BlockPit() -> impl IntoView {
    let container_ref = NodeRef::<Div>::new();
    
    // --- SKILLS ---
    let core_skills = vec!["RUST", "WASM", "LEPTOS", "SOLANA", "TOKIO"];
    let block_count = core_skills.len(); 
    let secondary_skills = "GIT • DOCKER • POSTGRES • TAILWIND • LINUX • AWS • TYPESCRIPT • NGINX • CI/CD • ACTIX • AXUM • GRAPHQL • REST • ";

    // --- PHYSICS CONSTANTS ---
    let gravity = 0.6;
    let friction = 0.95;
    let floor_friction = 0.90;
    let bounce = 0.15;
    let floor_y = 300.0;
    let sleep_threshold = 0.05;
    let block_size = 96.0; 
    let collision_dist = 115.0; 

    // --- STATE ---
    let objects = Rc::new(RefCell::new(
        (0..block_count).map(|i| {
            // MANUAL COORDINATES FOR LAYOUT
            let (start_x, start_y) = match i {
                // --- GROUP 1: CENTER PYRAMID (3 Blocks) ---
                0 => (550.0, floor_y - block_size - 5.0),          // Bottom Left
                1 => (550.0 + 100.0, floor_y - block_size - 5.0),  // Bottom Right
                2 => (550.0 + 50.0, floor_y - (block_size * 2.0) - 5.0), // Top Center
                
                // --- GROUP 2: EXTREME RIGHT STACK (2 Blocks) ---
                3 => (1000.0, floor_y - block_size - 5.0),         // Bottom
                4 => (1000.0, floor_y - (block_size * 2.0) - 5.0), // Top
                
                // Fallback (just in case you add more skills later)
                _ => (100.0 + (i as f64 * 50.0), 0.0),
            };

            PhysicsObject {
                x: start_x, 
                y: start_y, 
                vx: 0.0,
                vy: 0.0,
                width: block_size,
                height: block_size,
                rotation: 0.0,
                vr: 0.0,
                is_sleeping: false,
            }
        }).collect::<Vec<_>>()
    ));

    let mouse_pos = Rc::new(RefCell::new((-1000.0, -1000.0)));
    let objects_for_effect = objects.clone();
    let mouse_pos_for_effect = mouse_pos.clone();

    // --- PHYSICS LOOP ---
    Effect::new(move |_| {
        let objects = objects_for_effect.clone();
        let mouse_pos = mouse_pos_for_effect.clone();
        
        let get_container_width = move || -> f64 {
            if let Some(div) = container_ref.get_untracked() {
                 return div.offset_width() as f64;
            }
            1200.0
        };

        let f: Rc<RefCell<Option<Box<dyn Fn()>>>> = Rc::new(RefCell::new(None));
        let g = f.clone(); 

        *g.borrow_mut() = Some(Box::new(move || {
            let mut objs = objects.borrow_mut();
            let (mx, my) = *mouse_pos.borrow();
            let container_width = get_container_width();

            for i in 0..block_count {
                let dx = objs[i].x - mx;
                let dy = objs[i].y - my;
                let dist = (dx*dx + dy*dy).sqrt();
                let kick_radius = 140.0; 

                // Mouse Interaction
                if dist < kick_radius {
                    objs[i].is_sleeping = false;
                    let force = (kick_radius - dist) / kick_radius;
                    let kick_power = 2.0;
                    objs[i].vx += (dx / dist) * kick_power * force * 5.0;
                    objs[i].vy += (dy / dist) * kick_power * force * 5.0;
                    objs[i].vr += (dx / dist) * 1.5; 
                }

                if objs[i].is_sleeping { continue; }

                // Physics Steps
                objs[i].vy += gravity;
                objs[i].x += objs[i].vx;
                objs[i].y += objs[i].vy;
                objs[i].rotation += objs[i].vr;
                objs[i].vx *= friction;
                objs[i].vy *= friction;
                objs[i].vr *= 0.92; 

                // Floor Collision
                if objs[i].y > floor_y - objs[i].height {
                    objs[i].y = floor_y - objs[i].height;
                    objs[i].vy *= -bounce;
                    objs[i].vx *= floor_friction; 
                    
                    if objs[i].vy.abs() < sleep_threshold && objs[i].vx.abs() < sleep_threshold {
                        objs[i].vy = 0.0; objs[i].vx = 0.0; objs[i].vr = 0.0; objs[i].is_sleeping = true; 
                    }
                }

                // Wall Collision
                if objs[i].x < 0.0 { objs[i].x = 0.0; objs[i].vx *= -0.5; }
                if objs[i].x > container_width - objs[i].width { objs[i].x = container_width - objs[i].width; objs[i].vx *= -0.5; }
            }

            // Block-to-Block Collision
            for i in 0..block_count {
                for j in (i + 1)..block_count {
                    let dx = objs[i].x - objs[j].x;
                    let dy = objs[i].y - objs[j].y;
                    let distance = (dx*dx + dy*dy).sqrt();
                    let min_dist = collision_dist;

                    if distance < min_dist {
                        objs[i].is_sleeping = false; objs[j].is_sleeping = false;
                        
                        let overlap = min_dist - distance;
                        let push_x = (dx / distance) * overlap * 0.5;
                        let push_y = (dy / distance) * overlap * 0.5;
                        
                        objs[i].x += push_x; objs[i].y += push_y;
                        objs[j].x -= push_x; objs[j].y -= push_y;
                        
                        objs[i].vx += push_x * 0.2; objs[i].vy += push_y * 0.2;
                        objs[j].vx -= push_x * 0.2; objs[j].vy -= push_y * 0.2;
                    }
                }
            }

            // Render
            for i in 0..block_count {
                if let Some(el) = document().get_element_by_id(&format!("phys-block-{}", i)) {
                   if let Ok(el) = el.dyn_into::<HtmlElement>() {
                       let _ = el.style().set_property("transform", 
                           &format!("translate({:.1}px, {:.1}px) rotate({:.1}deg)", 
                           objs[i].x, objs[i].y, objs[i].rotation));
                   }
                }
            }

            let recursive_f = f.clone();
            request_animation_frame(move || {
                if let Some(callback) = recursive_f.borrow().as_ref() { callback(); }
            });
        }));

        let start_f = g.clone();
        request_animation_frame(move || {
            if let Some(callback) = start_f.borrow().as_ref() { callback(); }
        });
    });

    let mouse_pos_for_move = mouse_pos.clone();
    let handle_mousemove = move |ev: MouseEvent| {
         if let Some(container) = container_ref.get() {
            let rect = container.get_bounding_client_rect();
            let mut m = mouse_pos_for_move.borrow_mut();
            m.0 = ev.client_x() as f64 - rect.left();
            m.1 = ev.client_y() as f64 - rect.top();
         }
    };

    let mouse_pos_for_leave = mouse_pos.clone();
    let handle_mouseleave = move |_| {
         let mut m = mouse_pos_for_leave.borrow_mut();
         *m = (-1000.0, -1000.0);
    };

    view! {
        <div 
            node_ref=container_ref
            class="w-full h-[300px] relative mt-20 mb-0 z-20 cursor-default border-b-2 border-zinc-800"
            on:mousemove=handle_mousemove
            on:mouseleave=handle_mouseleave
        >
             <style>
                "@keyframes marquee {
                    0% { transform: translateX(0); }
                    100% { transform: translateX(-50%); }
                }"
            </style>

            <div class="absolute inset-0 overflow-hidden pointer-events-none z-0">
                <div class="absolute top-10 left-0 w-[200%] opacity-20">
                    <div 
                        class="whitespace-nowrap text-4xl md:text-6xl font-black text-zinc-500"
                        style="animation: marquee 30s linear infinite;"
                    >
                        <span>{secondary_skills} {secondary_skills}</span>
                    </div>
                    
                    <div 
                        class="whitespace-nowrap text-2xl md:text-3xl font-mono text-zinc-600 mt-4"
                        style="animation: marquee 45s linear infinite reverse;"
                    >
                        <span>"SYSTEMS PROGRAMMING • WEB ASSEMBLY • FULL STACK DEVELOPMENT • "</span>
                        <span>"SYSTEMS PROGRAMMING • WEB ASSEMBLY • FULL STACK DEVELOPMENT • "</span>
                    </div>
                </div>
            </div>

            <div class="absolute bottom-2 left-4 text-zinc-800 font-bold text-6xl opacity-30 pointer-events-none select-none z-10">
                "CORE_STACK"
            </div>

            {(0..block_count).map(|i| {
                view! {
                    <div 
                        id=format!("phys-block-{}", i)
                        class="absolute top-0 left-0 w-24 h-24 bg-zinc-900 border-2 border-orange-500/50 rounded-lg flex items-center justify-center shadow-[0_0_20px_rgba(249,115,22,0.15)] select-none z-20"
                        style="will-change: transform;" 
                    >
                        <div class="absolute inset-0 opacity-20 bg-[url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNCIgaGVpZ2h0PSI0IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPjxjaXJjbGUgY3g9IjIiIGN5PSIyIiByPSIxIiBmaWxsPSIjMzMzIiAvPjwvc3ZnPg==')]"></div>
                        
                        <span class="font-black text-xl tracking-wider relative z-30 text-orange-500 drop-shadow-[0_0_8px_rgba(249,115,22,0.8)]" 
                              style="font-family: monospace;">
                            {core_skills[i]}
                        </span>
                        
                        <div class="absolute top-1.5 left-1.5 w-1.5 h-1.5 bg-zinc-700 rounded-full"></div>
                        <div class="absolute top-1.5 right-1.5 w-1.5 h-1.5 bg-zinc-700 rounded-full"></div>
                        <div class="absolute bottom-1.5 left-1.5 w-1.5 h-1.5 bg-zinc-700 rounded-full"></div>
                        <div class="absolute bottom-1.5 right-1.5 w-1.5 h-1.5 bg-zinc-700 rounded-full"></div>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}