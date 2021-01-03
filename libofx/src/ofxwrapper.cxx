#include "libofx/libofx.h"
#include <cerrno>
#include <stdexcept>

extern "C" {
	LibofxContextPtr libofx_get_new_context_noexcept() noexcept(true) {
		try {
			return libofx_get_new_context();
		} catch (...) {
			return NULL;
		}
	}

	int libofx_proc_file_noexcept(
			LibofxContextPtr libofx_context,
			const char * p_filename,
			enum LibofxFileFormat ftype) noexcept(true)
	{
		try {
			return libofx_proc_file(libofx_context, p_filename, ftype);
		} catch (std::bad_alloc &) {
			return ENOMEM;
		} catch (std::range_error &) {
			return ERANGE;
		} catch (...) {
			return EINVAL;
		}
	}
}
