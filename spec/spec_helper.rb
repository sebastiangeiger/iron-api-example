require 'capybara/rspec'
require 'capybara/mechanize'

Capybara.register_driver :mechanize do |app|
  Capybara::Mechanize::Driver.new(app)
end

Capybara.configure do |config|
  config.run_server = false
  config.default_driver = :mechanize
  config.app_host = "http://localhost:3000"
  config.app = true # This is required :-/
end

RSpec.configure do |config|
  config.before(:all) do
    raise "`cargo build` failed" unless system("cargo build")
    @server_pid = fork do
      exec "target/debug/iron-api-example"
    end
  end

  config.after(:all) do
    Process.kill("INT", @server_pid)
    Process.wait
  end
end

