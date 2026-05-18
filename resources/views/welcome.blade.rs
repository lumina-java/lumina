<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Welcome to {{ app_name }}</title>
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
            --primary-glow: radial-gradient(circle, rgba(99, 102, 241, 0.15) 0%, transparent 60%);
            --secondary-glow: radial-gradient(circle, rgba(236, 72, 153, 0.12) 0%, transparent 60%);
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
            left: -10%;
            width: 60vw;
            height: 60vw;
            background: var(--primary-glow);
            z-index: 0;
            pointer-events: none;
        }

        .glow-2 {
            position: absolute;
            bottom: -20%;
            right: -10%;
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
            background: linear-gradient(135deg, #818cf8 0%, #ec4899 100%);
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
            max-width: 800px;
            z-index: 10;
        }

        .badge {
            background: rgba(99, 102, 241, 0.1);
            border: 1px solid rgba(99, 102, 241, 0.3);
            color: #a5b4fc;
            padding: 0.4rem 1rem;
            border-radius: 9999px;
            font-size: 0.85rem;
            font-weight: 600;
            margin-bottom: 2rem;
            letter-spacing: 0.05em;
            text-transform: uppercase;
            animation: pulse 2s infinite alternate;
        }

        h1 {
            font-family: 'Space Grotesk', sans-serif;
            font-size: 4rem;
            font-weight: 700;
            line-height: 1.1;
            letter-spacing: -0.03em;
            margin-bottom: 1.5rem;
            background: linear-gradient(to right, #ffffff 30%, #a5b4fc 70%, #ec4899 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }

        @media (max-width: 768px) {
            h1 {
                font-size: 2.5rem;
            }
        }

        .lead {
            font-size: 1.25rem;
            color: var(--text-secondary);
            max-width: 600px;
            line-height: 1.6;
            margin-bottom: 3rem;
        }

        /* Action Buttons */
        .actions {
            display: flex;
            gap: 1rem;
            margin-bottom: 4rem;
        }

        .btn {
            display: inline-block;
            padding: 0.8rem 2rem;
            border-radius: 12px;
            font-size: 1rem;
            font-weight: 600;
            text-decoration: none;
            transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
            cursor: pointer;
        }

        .btn-primary {
            background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
            color: white;
            box-shadow: 0 4px 20px rgba(99, 102, 241, 0.3);
        }

        .btn-primary:hover {
            transform: translateY(-2px);
            box-shadow: 0 6px 24px rgba(99, 102, 241, 0.45);
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

        /* Features Cards Grid */
        .features {
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 1.5rem;
            width: 100%;
            max-width: 900px;
        }

        @media (max-width: 768px) {
            .features {
                grid-template-columns: 1fr;
            }
        }

        .card {
            background: var(--card-bg);
            border: 1px solid var(--card-border);
            padding: 2rem;
            border-radius: 16px;
            text-align: left;
            backdrop-filter: blur(12px);
            transition: all 0.3s ease;
        }

        .card:hover {
            border-color: rgba(99, 102, 241, 0.4);
            transform: translateY(-4px);
            box-shadow: 0 10px 30px rgba(0, 0, 0, 0.4);
        }

        .card-icon {
            font-size: 1.8rem;
            margin-bottom: 1rem;
        }

        .card h3 {
            font-family: 'Space Grotesk', sans-serif;
            font-size: 1.25rem;
            margin-bottom: 0.5rem;
        }

        .card p {
            font-size: 0.9rem;
            color: var(--text-secondary);
            line-height: 1.5;
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

        @keyframes pulse {
            0% { opacity: 0.8; }
            100% { opacity: 1; }
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
        <div class="badge">🚀 v{{ version }} is live</div>
        <h1>Meet {{ app_name }}</h1>
        <p class="lead">The Premium, Highly Ergonomic Rust Framework for Elegant & Modern Backend Developers.</p>

        <div class="actions">
            <a href="/about" class="btn btn-primary">Learn About Lumina</a>
            <a href="https://github.com/lumina-java/framework" target="_blank" class="btn btn-secondary">Explore GitHub</a>
        </div>

        <div class="features">
            <div class="card">
                <div class="card-icon">🦀</div>
                <h3>Rust Speed</h3>
                <p>Built directly on the safest language in the world, achieving blistering speed without memory compromise.</p>
            </div>
            <div class="card">
                <div class="card-icon">⚡</div>
                <h3>Axum Engine</h3>
                <p>Fully utilizing Axum & Tokio underneath to offer production-hardened concurrent networking capabilities.</p>
            </div>
            <div class="card">
                <div class="card-icon">🎨</div>
                <h3>Blade Templates</h3>
                <p>Expressive blade-inspired templates rendered natively on Rust for zero-cost reactive static page experiences.</p>
            </div>
        </div>
    </main>

    <footer>
        <div>© 2026 Lumina Engine. All rights reserved.</div>
        <div>Engineered with ❤️ in Rust.</div>
    </footer>
</body>
</html>
