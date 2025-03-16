#include "arguments.hpp"

Arguments::Arguments()
{
}

Arguments::~Arguments()
{

}

void Arguments::Parse(int argc, char** argv)
{
    argparse::ArgumentParser program(PROGRAM_NAME);

    program.add_argument("filename").help("Input file name");
    program.add_argument("-v", "--verbose").default_value(false).implicit_value(true);

    try {
        program.parse_args(argc, argv);
    } catch (const std::exception &err) {
        std::cerr << "Argument error: " << err.what() << "\n\n";
        std::cerr << program;
        return;
    }

    std::cout << "Filename: " << program.get<std::string>("filename") << std::endl;
}
