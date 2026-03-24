@echo off
:: Mcalculator Tool Script
title Mcalculator Build and Dev Tools
setlocal
echo.
echo ========================================
echo   Mcalculator Build and Dev Tools
echo ========================================
echo.
echo [1] Full Build   (npm run build)
echo [2] Fast Build   (npm run build:rust)
echo [3] Dev Run      (npm run tauri dev)
echo [4] Clean Build  (cargo clean)
echo.
set /p "choice=Please select [1/2/3/4]: "

if "%choice%"=="1" goto :CASE_1
if "%choice%"=="2" goto :CASE_2
if "%choice%"=="3" goto :CASE_3
if "%choice%"=="4" goto :CASE_4
goto :DEFAULT

:CASE_1
    echo [1] Running full build...
    npm run build
    goto :END

:CASE_2
    echo [2] Running fast build...
    npm run build:rust
    goto :END

:CASE_3
    echo [3] Starting dev mode...
    npm run tauri dev
    goto :END

:CASE_4
    echo [4] Cleaning build artifacts...
    cd src-tauri && cargo clean
    echo Clean finished.
    goto :END

:DEFAULT
    echo Invalid selection.
    goto :END

:END
echo.
echo Operation finished.
pause
