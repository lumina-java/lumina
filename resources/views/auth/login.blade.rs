@extends('layouts.app')

@section('content')
<div class="min-h-[75vh] flex items-center justify-center px-4 relative">
    <div class="absolute w-[400px] h-[400px] bg-indigo-500/5 rounded-full blur-[100px] pointer-events-none"></div>

    <div class="max-w-md w-full bg-slate-900/60 border border-slate-800/80 rounded-3xl p-10 backdrop-filter backdrop-blur-md shadow-2xl relative z-10">
        <div class="text-center mb-8">
            <h2 class="text-3xl font-extrabold heading-font text-white">Welcome Back</h2>
            <p class="text-slate-400 mt-2 text-sm">Enter your credentials to manage your account</p>
        </div>
        
        <form action="/login" method="POST" class="space-y-6">
            @csrf
            <div>
                <label class="block text-sm font-semibold text-slate-300 mb-2">Email Address</label>
                <input type="email" name="email" value="{{ old('email') }}" class="w-full bg-slate-950/80 border border-slate-800 rounded-xl px-4 py-3 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition" placeholder="you@example.com" required>
                @error('email')
                <span class="text-xs text-rose-400 mt-1.5 block font-medium">{{ $message }}</span>
                @enderror
            </div>
            <div>
                <label class="block text-sm font-semibold text-slate-300 mb-2">Password</label>
                <input type="password" name="password" class="w-full bg-slate-950/80 border border-slate-800 rounded-xl px-4 py-3 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition" placeholder="••••••••" required>
                @error('password')
                <span class="text-xs text-rose-400 mt-1.5 block font-medium">{{ $message }}</span>
                @enderror
            </div>
            <button type="submit" class="w-full bg-indigo-600 text-white py-3.5 rounded-xl font-bold hover:bg-indigo-500 shadow-lg shadow-indigo-600/20 transition transform active:scale-98">
                Sign In
            </button>
        </form>
        
        <p class="text-center text-sm text-slate-400 mt-8">
            Don't have an account? <a href="/register" class="text-indigo-400 font-bold hover:underline">Register</a>
        </p>
    </div>
</div>
@endsection