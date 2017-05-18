#include "magic.h"
#include <string>
#include <iostream>

using namespace std;

// Need more info on this
// Without this extern declaration i get `undefined symbol: __dso_handle`
// while linking.
extern "C"
{
    void *__dso_handle = NULL;
}
 
class MathObject {
    protected:
        std::string m_name;
        MathObject(std::string name) : m_name(name) {}
    public:
        std::string getName() { return m_name; }
        virtual size_t add(size_t a, size_t b) = 0;
};

class Adder: public MathObject {
    public:
        Adder(std::string name) : MathObject(name) {}
        size_t add(size_t a, size_t b) {
            return a+b;
        }
};

// The function available in rust
size_t add(size_t a, size_t b) {
  Adder some_adder("myadder");
  return some_adder.add(a, b);
}
