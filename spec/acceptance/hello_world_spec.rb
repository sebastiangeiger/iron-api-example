require_relative '../../spec/spec_helper'

RSpec.describe '/', type: :feature do
  it 'gets hello world' do
    visit('/')
    expect(page).to have_content "Hello World"
  end

  it 'does not get goodbye' do
    visit('/')
    expect(page).to_not have_content "Goodbye!"
  end
end
