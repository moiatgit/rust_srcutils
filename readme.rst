This is a bunch of utilities to help me manage different languages source
files.

Project status and license of use
=================================

Currently this project is in development.

This is my first project in Rust, and I'm using it to learn the language
and ecosystem.

You can use what you find useful here on your own risk.

Suggestions are welcomed.

Command line tools
==================

The tools (plural?) already available are:

``javaheaders``
---------------

Usage: ::

    $ javaheaders path.java > /tmp/headerofpath


Library
=======

The library contains the following functions (plural?):

``extract_java_headers()``
--------------------------

This function receives a String with a java source code and returns it's
headers if any.

It resolves it with the following state diagram:

.. image:: doc/javaheader.png

