from setuptools import setup, Extension
from setuptools.command.build_ext import build_ext
import os
import subprocess
import shutil

class MakeBuild(build_ext):
    def run(self):
        cwd = os.getcwd()
        c_src_dir = os.path.join(cwd, 'src', 'fft-c')
        build_lib_dir = os.path.join(cwd, 'build', 'lib')

        os.chdir(c_src_dir)
        try:
            print("Building FFT C library...")
            subprocess.check_call(['mingw32-make'])
            print("Build complete.")

            print("Copying FFT library to build directory...")
            subprocess.check_call(['mingw32-make', 'install'])
            print("Copy complete.")
        except subprocess.CalledProcessError as e:
            raise RuntimeError('Kompilering med mingw32-make misslyckades: {}'.format(e))
        finally:
            os.chdir(cwd)
        
    def copy_extensions_to_release(self):
        pass

fft_module = Extension(
    'fft',
    sources=[],
    include_dirs=[os.path.join('src', 'fft-c', '')],
    library_dirs=[os.path.join('build', 'lib')],
    libraries=['fft'],
)

setup(
    name='fft',
    version='0.1.0',
    description='Snabb fouriertransform byggd med Makefile (MinGW)',
    ext_modules=[fft_module],
    cmdclass={'build_ext': MakeBuild},
    package_dir={'': 'build/lib'},
    package_data={'': ['*.pyd', '*.so']}, # Inkludera .pyd och .so filer
)