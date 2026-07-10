<?php

declare(strict_types=1);

use App\Http\Controllers\ProjectsController;
use Illuminate\Support\Facades\Route;

Route::get('/', ProjectsController::class)->name('home');
