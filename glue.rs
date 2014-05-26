/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/. */

/* automatically generated by rust-bindgen */

use libc;
use jsapi::*;
use jsfriendapi::JSJitInfo;
use jsval::JSVal;

type c_bool = libc::c_int;

pub struct ProxyTraps {
    pub getPropertyDescriptor: Option<extern "C" fn(*mut JSContext, *mut JSObject, jsid, c_bool, *mut JSPropertyDescriptor) -> c_bool>,
    pub getOwnPropertyDescriptor: Option<extern "C" fn(*mut JSContext, *mut JSObject, jsid, JSBool, *mut JSPropertyDescriptor) -> JSBool>,
    pub defineProperty: Option<extern "C" fn(*mut JSContext, *mut JSObject, jsid, *JSPropertyDescriptor) -> JSBool>,
    pub getOwnPropertyNames: *u8, //XXX need a representation for AutoIdVector&
    pub delete_: Option<extern "C" fn(*mut JSContext, *mut JSObject, jsid, *mut bool) -> JSBool>,
    pub enumerate: *u8, //XXX need a representation for AutoIdVector&

    pub has: Option<extern "C" fn(*mut JSContext, *mut JSObject, jsid, *mut JSBool) -> JSBool>,
    pub hasOwn: Option<extern "C" fn(*mut JSContext, *mut JSObject, jsid, *mut JSBool) -> JSBool>,
    pub get: Option<extern "C" fn(*mut JSContext, *mut JSObject, *mut JSObject, jsid, *mut JSVal) -> JSBool>,
    pub set: Option<extern "C" fn(*mut JSContext, *mut JSObject, *mut JSObject, jsid, JSBool, *mut JSVal) -> JSBool>,
    pub keys: *u8, //XXX need a representation for AutoIdVector&
    pub iterate: Option<extern "C" fn(*mut JSContext, *mut JSObject, uint, *mut JSVal) -> JSBool>,

    pub call: Option<extern "C" fn(*mut JSContext, *mut JSObject, uint, *mut JSVal) -> JSBool>,
    pub construct: Option<extern "C" fn(*mut JSContext, *mut JSObject, uint, *mut JSVal, *mut JSVal) -> JSBool>,
    pub nativeCall: *u8, //XXX need a representation for IsAcceptableThis, NativeImpl, and CallArgs
    pub hasInstance: Option<extern "C" fn(*mut JSContext, *mut JSObject, *mut JSVal, *mut JSBool) -> JSBool>,
    pub typeOf: Option<extern "C" fn(*mut JSContext, *mut JSObject) -> uint>, //XXX JSType enum
    pub objectClassIs: Option<extern "C" fn(*mut JSObject, uint, *mut JSContext) -> JSBool>, //XXX ESClassValue enum
    pub obj_toString: Option<extern "C" fn(*mut JSContext, *mut JSObject) -> *mut JSString>,
    pub fun_toString: Option<extern "C" fn(*mut JSContext, *mut JSObject, uint) -> *mut JSString>,
    //regexp_toShared: *u8,
    pub defaultValue: Option<extern "C" fn(*mut JSContext, *mut JSObject, uint, *mut JSVal) -> JSBool>, //XXX JSType enum
    pub iteratorNext: Option<extern "C" fn(*mut JSContext, *mut JSObject, *mut JSVal) -> JSBool>,
    pub finalize: Option<extern "C" fn(*mut JSFreeOp, *mut JSObject)>,
    pub getElementIfPresent: Option<extern "C" fn(*mut JSContext, *mut JSObject, *mut JSObject, u32, *mut JSVal, *mut JSBool) -> JSBool>,
    pub getPrototypeOf: Option<extern "C" fn(*mut JSContext, *mut JSObject, **mut JSObject) -> JSBool>,
    pub trace: Option<extern "C" fn(*mut JSTracer, *mut JSObject)>,
}

#[link(name = "jsglue")]
extern { }


#[cfg(target_os = "android")]
#[link_args = "-ljsglue -lstdc++ -lgcc -rdynamic"]
extern { }

extern {

//#[rust_stack]
pub fn RUST_JS_NumberValue(d: f64) -> JSVal;

//#[rust_stack]
pub fn CallJitPropertyOp(info: *JSJitInfo, cx: *mut JSContext, thisObj: *mut JSObject, specializedThis: *libc::c_void, vp: *mut JSVal) -> JSBool;

//#[rust_stack]
pub fn CallJitMethodOp(info: *JSJitInfo, cx: *mut JSContext, thisObj: *mut JSObject, specializedThis: *libc::c_void, argc: libc::c_uint, vp: *mut JSVal) -> JSBool;

//#[rust_stack]
pub fn RUST_FUNCTION_VALUE_TO_JITINFO(v: JSVal) -> *JSJitInfo;

pub fn SetFunctionNativeReserved(fun: *mut JSObject, which: libc::size_t, val: *mut JSVal);
pub fn GetFunctionNativeReserved(fun: *mut JSObject, which: libc::size_t) -> *mut JSVal;

pub fn CreateProxyHandler(traps: *ProxyTraps, extra: *libc::c_void) -> *libc::c_void;
pub fn CreateWrapperProxyHandler(traps: *ProxyTraps) -> *libc::c_void;
pub fn NewProxyObject(cx: *mut JSContext, handler: *libc::c_void, priv_: *mut JSVal,
                      proto: *mut JSObject, parent: *mut JSObject, call: *mut JSObject,
                      construct: *mut JSObject) -> *mut JSObject;
pub fn WrapperNew(cx: *mut JSContext, parent: *mut JSObject, handler: *libc::c_void) -> *mut JSObject;

pub fn GetProxyExtra(obj: *mut JSObject, slot: libc::c_uint) -> JSVal;
pub fn GetProxyPrivate(obj: *mut JSObject) -> JSVal;
pub fn SetProxyExtra(obj: *mut JSObject, slot: libc::c_uint, val: JSVal);

pub fn GetObjectProto(obj: *mut JSObject) -> *mut JSObject;
pub fn GetObjectParent(obj: *mut JSObject) -> *mut JSObject;

pub fn RUST_JSID_IS_INT(id: jsid) -> JSBool;
pub fn RUST_JSID_TO_INT(id: jsid) -> libc::c_int;
pub fn RUST_JSID_IS_STRING(id: jsid) -> JSBool;
pub fn RUST_JSID_TO_STRING(id: jsid) -> *mut JSString;

pub fn RUST_SET_JITINFO(func: *JSFunction, info: *JSJitInfo);

pub fn RUST_INTERNED_STRING_TO_JSID(cx: *mut JSContext, str: *mut JSString) -> jsid;

pub fn DefineFunctionWithReserved(cx: *mut JSContext, obj: *mut JSObject,
                                  name: *libc::c_char, call: JSNative, nargs: libc::c_uint,
                                  attrs: libc::c_uint) -> *mut JSObject;
pub fn GetObjectJSClass(obj: *mut JSObject) -> *JSClass;
pub fn RUST_js_GetErrorMessage(userRef: *libc::c_void, locale: *libc::c_char,
                               errorNumber: libc::c_uint) -> *JSErrorFormatString;
pub fn js_IsObjectProxyClass(obj: *mut JSObject) -> bool;
pub fn js_IsFunctionProxyClass(obj: *mut JSObject) -> bool;
pub fn IsProxyHandlerFamily(obj: *mut JSObject) -> bool;
pub fn GetProxyHandlerExtra(obj: *mut JSObject) -> *libc::c_void;
pub fn GetProxyHandler(obj: *mut JSObject) -> *libc::c_void;
pub fn InvokeGetOwnPropertyDescriptor(handler: *libc::c_void, cx: *mut JSContext, proxy: *mut JSObject, id: jsid, set: JSBool, desc: *mut JSPropertyDescriptor) -> JSBool;
pub fn GetGlobalForObjectCrossCompartment(obj: *mut JSObject) -> *mut JSObject;
pub fn ReportError(cx: *mut JSContext, error: *libc::c_char);
pub fn IsWrapper(obj: *mut JSObject) -> JSBool;
pub fn UnwrapObject(obj: *mut JSObject, stopAtOuter: JSBool, flags: *libc::c_uint) -> *mut JSObject;
}
