package ren.home.bingeAtHome.controller.util;

import ren.home.bingeAtHome.controller.handlers.WebRestControllerAdvice;

public class ErrorMsgJsonCreator {

    public static String get(Exception e) {
        return "{\"" + WebRestControllerAdvice.messageKey + "\":\"" + e.getMessage() + "\"}";
    }
}
