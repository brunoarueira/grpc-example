#!/usr/bin/env ruby
# frozen_string_literal: true

require 'bundler/inline'

gemfile do
  source 'https://rubygems.org'

  gem 'grpc', '1.41.0'
  gem 'grpc-tools'
end

lib_dir = File.expand_path(File.join(File.dirname(__dir__), 'lib'))
$LOAD_PATH.unshift(lib_dir) unless $LOAD_PATH.include?(lib_dir)

require 'calculator_services_pb'

server_url = ENV['SERVER_URL'] || '0.0.0.0:8080'
calculator = Calculator::Calculator::Stub.new(server_url, :this_channel_is_insecure)

first_value = 10
second_value = 2

input = Calculator::Input.new(first_value: first_value, second_value: second_value)

output = calculator.sum(input)

puts '~' * 90
puts "Sum result between #{first_value} and #{second_value} is:"
puts output.value
puts '~' * 90

output = calculator.minus(input)

puts '~' * 90
puts "Diff result between #{first_value} and #{second_value} is:"
puts output.value
puts '~' * 90
