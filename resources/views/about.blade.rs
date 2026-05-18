<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>About {{ app_name }}</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@300;400;500;600;700;800&family=Space+Grotesk:wght@400;500;600;700&display=swap" rel="stylesheet">
    <style>
        :root {
            --bg-color: #030712;
            --card-bg: rgba(17, 24, 39, 0.7);
            --card-border: rgba(255, 255, 255, 0.08);
            --text-primary: #f9fafb;
            --text-secondary: #9ca3af;
            --primary-glow: radial-gradient(circle, rgba(236, 72, 153, 0.15) 0%, transparent 60%);
            --secondary-glow: radial-gradient(circle, rgba(99, 102, 241, 0.12) 0%, transparent 60%);
        }

        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }

        body {
            font-family: 'Plus Jakarta Sans', sans-serif;
            background-color: var(--bg-color);
            color: var(--text-primary);
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            align-items: center;
            overflow-x: hidden;
            position: relative;
        }

        /* Ambient Glows */
        .glow-1 {
            position: absolute;
            top: -20%;
            right: -10%;
            width: 60vw;
            height: 60vw;
            background: var(--primary-glow);
            z-index: 0;
            pointer-events: none;
        }

        .glow-2 {
            position: absolute;
            bottom: -20%;
            left: -10%;
            width: 60vw;
            height: 60vw;
            background: var(--secondary-glow);
            z-index: 0;
            pointer-events: none;
        }

        /* Glassmorphism Navigation */
        header {
            width: 100%;
            max-width: 1200px;
            padding: 1.5rem 2rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
            z-index: 10;
        }

        .logo {
            font-family: 'Space Grotesk', sans-serif;
            font-size: 1.5rem;
            font-weight: 700;
            background: linear-gradient(135deg, #ec4899 0%, #818cf8 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            letter-spacing: -0.05em;
        }

        .nav-links a {
            color: var(--text-secondary);
            text-decoration: none;
            font-size: 0.95rem;
            font-weight: 500;
            margin-left: 2rem;
            transition: color 0.3s ease;
        }

        .nav-links a:hover {
            color: var(--text-primary);
        }

        /* Hero Area */
        main {
            display: flex;
            flex-direction: column;
            align-items: center;
            text-align: center;
            padding: 4rem 2rem;
            max-width: 850px;
            z-index: 10;
        }

        h1 {
            font-family: 'Space Grotesk', sans-serif;
            font-size: 3.5rem;
            font-weight: 700;
            line-height: 1.1;
            letter-spacing: -0.03em;
            margin-bottom: 1.5rem;
            background: linear-gradient(to right, #ffffff 40%, #f472b6 80%, #818cf8 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }

        @media (max-width: 768px) {
            h1 {
                font-size: 2.5rem;
            }
        }

        .philosophy {
            background: var(--card-bg);
            border: 1px solid var(--card-border);
            padding: 2.5rem;
            border-radius: 24px;
            backdrop-filter: blur(12px);
            margin-bottom: 3rem;
            text-align: left;
            box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
        }

        .philosophy h2 {
            font-family: 'Space Grotesk', sans-serif;
            font-size: 1.75rem;
            margin-bottom: 1rem;
            color: #f472b6;
        }

        .philosophy p {
            font-size: 1.1rem;
            color: var(--text-secondary);
            line-height: 1.7;
            margin-bottom: 1.5rem;
        }

        /* Pillars grid */
        .pillars {
            display: grid;
            grid-template-columns: repeat(2, 1fr);
            gap: 1.5rem;
            width: 100%;
        }

        @media (max-width: 768px) {
            .pillars {
                grid-template-columns: 1fr;
            }
        }

        .pillar {
            background: rgba(255, 255, 255, 0.02);
            border: 1px solid rgba(255, 255, 255, 0.05);
            padding: 1.5rem;
            border-radius: 16px;
            transition: all 0.3s ease;
        }

        .pillar:hover {
            background: rgba(255, 255, 255, 0.04);
            border-color: rgba(244, 114, 182, 0.3);
        }

        .pillar h3 {
            font-family: 'Space Grotesk', sans-serif;
            font-size: 1.2rem;
            margin-bottom: 0.5rem;
            color: var(--text-primary);
        }

        .pillar p {
            font-size: 0.95rem;
            color: var(--text-secondary);
            line-height: 1.5;
        }

        /* Actions */
        .actions {
            margin-top: 3rem;
        }

        .btn {
            display: inline-block;
            padding: 0.8rem 2.5rem;
            border-radius: 12px;
            font-size: 1rem;
            font-weight: 600;
            text-decoration: none;
            transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
            cursor: pointer;
        }

        .btn-secondary {
            background: var(--card-bg);
            border: 1px solid var(--card-border);
            color: var(--text-primary);
            backdrop-filter: blur(10px);
        }

        .btn-secondary:hover {
            background: rgba(255, 255, 255, 0.05);
            border-color: rgba(255, 255, 255, 0.2);
            transform: translateY(-2px);
        }

        /* Footer */
        footer {
            width: 100%;
            max-width: 1200px;
            padding: 2rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
            border-top: 1px solid rgba(255, 255, 255, 0.05);
            z-index: 10;
            font-size: 0.85rem;
            color: var(--text-secondary);
        }
    </style>
</head>
<body>
    <div class="glow-1"></div>
    <div class="glow-2"></div>

    <header>
        <div class="logo">Lumina</div>
        <nav class="nav-links">
            <a href="/">Home</a>
            <a href="/about">About</a>
        </nav>
    </header>

    <main>
        <h1>About the Framework</h1>

        <div class="philosophy">
            <h2>The Lumina Philosophy</h2>
            <p>
                Lumina was born out of a desire to bring the elegant, rapid-development experience of frameworks like Laravel and Rails directly to the ultra-efficient, safe world of Rust.
            </p>
            <p>
                We believe developers shouldn't have to choose between speed of development and execution performance. Lumina combines ergonomic MVC architecture with Rust's uncompromising safety and performance.
            </p>

            <div class="pillars">
                <div class="pillar">
                    <h3>🎨 Dev-First Ergonomics</h3>
                    <p>Designed with intuitive routing, clear request-handling structures, and a beautiful templating engine, Lumina feels familiar from day one.</p>
                </div>
                <div class="pillar">
                    <h3>🔒 Strict Type Safety</h3>
                    <p>Leveraging Rust's compiler to prevent runtime errors, database mismatches, and race conditions before your app even builds.</p>
                </div>
                <div class="pillar">
                    <h3>🚀 Non-blocking Performance</h3>
                    <p>Driven by Tokio's async runtime, handling tens of thousands of concurrent requests seamlessly on minimal server memory.</p>
                </div>
                <div class="pillar">
                    <h3>📦 All-in-One Engine</h3>
                    <p>Includes built-in migrations, routing, and blade-like static rendering out of the box, avoiding chaotic dependencies.</p>
                </div>
            </div>
        </div>

        <div class="actions">
            <a href="/" class="btn btn-secondary">← Back to Welcome</a>
        </div>
    </main>

    <footer>
        <div>© 2026 Lumina Engine. All rights reserved.</div>
        <div>Engineered with ❤️ in Rust.</div>
    </footer>
</body>
</html>
