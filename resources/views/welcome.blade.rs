<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ app_name }}</title>
    <style>
        body {
            font-family: 'Inter', sans-serif;
            background-color: #0f172a;
            color: #f8fafc;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            margin: 0;
        }
        .container {
            text-align: center;
        }
        h1 {
            font-size: 4rem;
            background: linear-gradient(to right, #38bdf8, #818cf8);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            margin-bottom: 0.5rem;
        }
        p {
            font-size: 1.2rem;
            color: #94a3b8;
        }
        .version {
            margin-top: 2rem;
            font-size: 0.9rem;
            color: #475569;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>{{ app_name }}</h1>
        <p>The Premium Rust Framework for Elegant Developers.</p>
        <div class="version">Version {{ version }}</div>
    </div>
</body>
</html>
