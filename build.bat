if exist bin\ (
  cd bin
) else (
  mkdir bin
)

cargo run
main.exe
pause