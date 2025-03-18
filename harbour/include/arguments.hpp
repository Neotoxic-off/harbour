#pragma once

#include <iostream>
#include <filesystem>

#include "spdlog/spdlog.h"
#include <argparse/argparse.hpp>

#include "settings.hpp"

struct Argument
{
    
};

class Arguments
{
    public:
        argparse::ArgumentParser program;

        Arguments();
        ~Arguments();
    
        bool Parse(int argc, char** argv);
};
