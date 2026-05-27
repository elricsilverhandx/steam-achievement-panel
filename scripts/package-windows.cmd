@echo off
set TARGET=%1
set ARTIFACT=%2
if exist dist rmdir /s /q dist
mkdir dist\%ARTIFACT%
copy target\%TARGET%\release\steam-achievement-panel.exe dist\%ARTIFACT%\steam-achievement-panel.exe
echo Steam Achievement Panel> dist\%ARTIFACT%\README.txt
tar -a -c -f dist\%ARTIFACT%.zip -C dist %ARTIFACT%
