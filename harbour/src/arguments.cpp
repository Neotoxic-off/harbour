#include "arguments.hpp"

Arguments::Arguments() : program(PROGRAM_NAME)
{
    this->program.add_argument("path").help("Project path");
}

Arguments::~Arguments()
{

}

bool Arguments::Parse(int argc, char** argv)
{
    try {
        this->program.parse_args(argc, argv);
    } catch (const std::exception &err) {
        spdlog::error("Argument error: {}", err.what());
        return false;
    }

    return true;
}
