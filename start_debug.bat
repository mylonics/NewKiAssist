@echo off
REM Startup script for KiAssist (Windows) - DEBUG VERSION

echo Starting KiAssist (Debug Mode)...

REM Create virtual environment if it doesn't exist
if not exist "venv\" (
    echo Creating Python virtual environment...
    python -m venv venv
    if errorlevel 1 (
        echo Error: Failed to create virtual environment. Make sure Python is installed.
        pause
        exit /b 1
    )
)

REM Activate virtual environment
call venv\Scripts\activate.bat
if errorlevel 1 (
    echo Error: Failed to activate virtual environment.
    pause
    exit /b 1
)

REM Build frontend
echo Building frontend...
call npm run build
if errorlevel 1 (
    echo Error: Failed to build frontend.
    pause
    exit /b 1
)

REM Install Python dependencies if needed
echo Checking Python dependencies...
cd python-lib
pip install -e . >nul 2>&1
if errorlevel 1 (
    echo Warning: Failed to install Python dependencies. Trying without --editable...
    pip install .
    if errorlevel 1 (
        echo Error: Failed to install Python dependencies.
        cd ..
        pause
        exit /b 1
    )
)
cd ..

REM Show Python version and packages for debugging
echo.
echo ===== Debug Information =====
python --version
echo Python executable: 
python -c "import sys; print(sys.executable)"
echo.
echo Checking keyring availability:
python -c "import sys; sys.path.insert(0, 'python-lib'); from kiassist_utils.api_key import ApiKeyStore; store = ApiKeyStore(); print('Keyring available:', store._is_keyring_available())"
echo.
echo Current API key status:
python -c "import sys; sys.path.insert(0, 'python-lib'); from kiassist_utils.api_key import ApiKeyStore; store = ApiKeyStore(); print('Has API key:', store.has_api_key())"
echo ============================
echo.

REM Run the Python application
echo Launching application...
echo (Watch for [DEBUG] messages in the console)
echo.
python -m kiassist_utils.main

echo.
echo Application exited. Press any key to close...
pause
