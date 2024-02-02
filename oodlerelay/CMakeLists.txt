cmake_minimum_required(VERSION 3.9)
project(liboodlerelay VERSION 1.0.0 DESCRIPTION "A static relay library to execute oodle dynamic library code")
add_library(oodlerelay STATIC src/oodlerelay.c)
install(TARGETS oodlerelay DESTINATION .)