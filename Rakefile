task default: :install_cli

task :install_cli do
  sh 'cargo build --release -p checklist-cli'
  sh 'mv ./target/release/cls ~/.local/bin'
end
