use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Samsung 65\" QLED 4K TV".to_string(),
            price: 1299.99,
            description: "Immerse yourself in true-to-life visuals and vibrant colors with the Samsung 65\" QLED 4K TV. Quantum Dot technology delivers stunning clarity, while its smart TV features provide access to all your favorite streaming apps.".to_string(),
            image: "/samsung_qled_tv.png".to_string()
        },
        Product {
            id: 2,
            name: "Apple AirPods Pro".to_string(),
            price: 249.99,
            description: "Experience the ultimate in wireless audio with Apple AirPods Pro. Enjoy active noise cancellation, customizable fit, and seamless connection to all your Apple devices. Perfect for music, calls, and workouts.".to_string(),
            image: "/apple_airpods_pro.png".to_string()
        },
        Product {
            id: 3,
            name: "Dell XPS 13 Laptop".to_string(),
            price: 999.99,
            description: "Work and play in style with the Dell XPS 13. Its sleek design, powerful Intel processor, and stunning InfinityEdge display make it ideal for multitasking, content creation, and entertainment on the go.".to_string(),
            image: "/dell_xps_13.png".to_string()
        },
        Product {
            id: 4,
            name: "Sony PlayStation 5".to_string(),
            price: 499.99,
            description: "Elevate your gaming experience with the PlayStation 5. Ultra-fast loading, adaptive triggers, and 3D audio technology immerse you in next-generation gameplay with incredibly realistic visuals.".to_string(),
            image: "/sony_ps5.png".to_string()
        },
        Product {
            id: 5,
            name: "Microsoft Xbox Wireless Controller".to_string(),
            price: 59.99,
            description: "Dominate your favorite games with the Microsoft Xbox Wireless Controller. Featuring an ergonomic design, textured triggers, and Bluetooth connectivity, it's perfect for console and PC gaming.".to_string(),
            image: "/xbox_controller.png".to_string()
        },
        Product {
            id: 6,
            name: "Bose QuietComfort 45 Headphones".to_string(),
            price: 329.99,
            description: "Block out distractions and focus on your music with Bose QuietComfort 45 headphones. Exceptional noise cancellation, comfortable earcups, and crystal-clear sound quality redefine your listening experience.".to_string(),
            image: "/bose_qc45.png".to_string()
        },
        Product {
            id: 7,
            name: "Logitech MX Master 3 Wireless Mouse".to_string(),
            price: 99.99,
            description: "Boost productivity with the Logitech MX Master 3. Its hyper-fast scrolling, ergonomic shape, and cross-computer control let you work seamlessly across multiple devices with effortless precision.".to_string(),
            image: "/logitech_mx_master_3.png".to_string()
        },
        Product {
            id: 8,
            name: "Amazon Echo Dot (4th Gen)".to_string(),
            price: 49.99,
            description: "Control your smart home, play music, and get answers with the Amazon Echo Dot. Featuring Alexa voice assistance, improved sound, and a sleek spherical design, it's the perfect addition to any room.".to_string(),
            image: "/echo_dot.png".to_string()
        },
        Product {
            id: 9,
            name: "Roku Streaming Stick 4K".to_string(),
            price: 39.99,
            description: "Enjoy seamless streaming with the Roku Streaming Stick 4K. It delivers cinematic picture quality, Dolby Vision, and voice search capabilities, offering quick access to your favorite shows and movies.".to_string(),
            image: "/roku_stick_4k.png".to_string()
        },
        Product {
            id: 10,
            name: "GoPro HERO10 Action Camera".to_string(),
            price: 399.99,
            description: "Capture every moment in stunning detail with the GoPro HERO10. Featuring HyperSmooth stabilization, 5.3K video resolution, and cloud connectivity, it's the ultimate tool for adventurers and content creators.".to_string(),
            image: "/gopro_hero10.png".to_string()
        }
    ]
}
