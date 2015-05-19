require_relative '../../spec/spec_helper'

describe 'API', type: :feature do
  let(:json) { JSON.parse(page.body) }

  describe 'GET /items' do
    it 'gets an empty list' do
      expect { visit('/items') }.to have_status_code 200
      expect(json).to eql [{"name" => "Bananas"}]
    end
  end

  describe 'GET /someotherroute' do
    it 'returns a 404' do
      expect { visit('/someotherroute') }.to have_status_code 404
    end
  end
end


