task default: :release

task :release do
  sh 'cargo build --release -p checklist-cli'
  sh 'mv ./target/release/cls ~/.local/bin'
end
