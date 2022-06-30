
#include <memory>

class Complex
{
public:
    Complex(double x, double y);
    double re;
    double im;
    double get_re() const;
    double get_im() const;
};

std::unique_ptr<Complex> new_comp(double re, double im);
