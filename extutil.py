# extutil.py - useful utility methods for extensions
#
# Copyright 2016 Facebook
#
# This software may be used and distributed according to the terms of the
# GNU General Public License version 2 or any later version.

def replaceclass(container, classname):
    '''Replace a class with another in a module, and interpose it into
    the hierarchies of all loaded subclasses. This function is
    intended for use as a decorator.

      import mymodule
      @replaceclass(mymodule, 'myclass')
      class mysubclass(mymodule.myclass):
          def foo(self):
              f = super(mysubclass, self).foo()
              return f + ' bar'

    Existing instances of the class being replaced will not have their
    __class__ modified, so call this function before creating any
    objects of the target type.
    '''
    def wrap(cls):
        oldcls = getattr(container, classname)
        for subcls in oldcls.__subclasses__():
            if subcls is not cls:
                assert oldcls in subcls.__bases__
                newbases = [oldbase
                            for oldbase in subcls.__bases__
                            if oldbase != oldcls]
                newbases.append(cls)
                subcls.__bases__ = tuple(newbases)
        setattr(container, classname, cls)
        return cls
    return wrap

def wrapfilecache(cls, propname, wrapper):
    """Wraps a filecache property. These can't be wrapped using the normal
    wrapfunction. This should eventually go into upstream Mercurial.
    """
    origcls = cls
    assert callable(wrapper)
    while cls is not object:
        if propname in cls.__dict__:
            origfn = cls.__dict__[propname].func
            assert callable(origfn)
            def wrap(*args, **kwargs):
                return wrapper(origfn, *args, **kwargs)
            cls.__dict__[propname].func = wrap
            break
        cls = cls.__bases__[0]

    if cls is object:
        raise AttributeError(_("type '%s' has no property '%s'") % (origcls,
                             propname))
