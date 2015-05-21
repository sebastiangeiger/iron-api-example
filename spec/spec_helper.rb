require 'faraday'
require 'json'

RSpec.configure do |config|
  config.before(:all) do
    raise "`cargo build` failed" unless system("cargo build")
    @server_pid = fork do
      exec "target/debug/iron-api-example"
    end
    sleep 0.1
  end

  config.after(:all) do
    Process.kill("INT", @server_pid)
    Process.wait
  end
end
