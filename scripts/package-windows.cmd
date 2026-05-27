@echo off
set TARGET=%1
set ARTIFACT=%2
if exist dist rmdir /s /q dist
mkdir dist\%ARTIFACT%
copy target\%TARGET%\release\steam-achievement-panel.exe dist\%ARTIFACT%\steam-achievement-panel.exe
for /r target\%TARGET%\release\build %%f in (steam_api64.dll) do copy "%%f" dist\%ARTIFACT%\steam_api64.dll
echo Steam Achievement Panel> dist\%ARTIFACT%\README.txt
echo Start Steam first, then run steam-achievement-panel.exe.>> dist\%ARTIFACT%\README.txt
tar -a -c -f dist\%ARTIFACT%.zip -C dist %ARTIFACT%
