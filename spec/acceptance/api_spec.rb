require_relative '../../spec/spec_helper'

describe 'API', type: :feature do
  describe 'GET /' do
    it 'gets hello world' do
      expect { visit('/') }.to have_status_code 200
      expect(page).to have_content "Hello World"
    end
  end

  describe 'GET /someotherroute' do
    it 'returns a 404' do
      expect { visit('/someotherroute') }.to have_status_code 404
    end
  end
end


