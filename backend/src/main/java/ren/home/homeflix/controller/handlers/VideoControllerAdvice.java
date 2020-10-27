package ren.home.homeflix.controller.handlers;

import lombok.extern.slf4j.Slf4j;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.RestControllerAdvice;
import ren.home.homeflix.service.exception.VideoMissingException;

import java.util.Collections;
import java.util.Map;

/**
 * Handles all exceptions with REST return values, thrown when a controller method encounters one.
 *
 * @author Attila Szőke
 */
@Slf4j
@RestControllerAdvice
public class VideoControllerAdvice {

    private static final String messageKey = "msg";

    @ExceptionHandler(VideoMissingException.class)
    public ResponseEntity<Map<String, Object>> handleException(VideoMissingException e) {
        log.error(e.getMessage());
        return ResponseEntity
                .status(HttpStatus.NOT_FOUND)
                .body(Collections.singletonMap(messageKey, e.getMessage()));
    }
}
