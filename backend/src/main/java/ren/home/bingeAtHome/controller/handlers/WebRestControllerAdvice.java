package ren.home.bingeAtHome.controller.handlers;

import lombok.extern.slf4j.Slf4j;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.validation.FieldError;
import org.springframework.web.bind.MethodArgumentNotValidException;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.RestControllerAdvice;
import ren.home.bingeAtHome.service.exception.MetadataCannotBeSavedException;
import ren.home.bingeAtHome.service.exception.VideoMissingException;

import java.util.Collections;
import java.util.HashMap;
import java.util.Map;

/**
 * Handles all exceptions with REST return values, thrown when a controller method encounters one.
 *
 * @author Attila Szőke
 */
@Slf4j
@RestControllerAdvice
public class WebRestControllerAdvice {

    public static final String messageKey = "err_msg";

    @ExceptionHandler(VideoMissingException.class)
    public ResponseEntity<Map<String, Object>> handleException(VideoMissingException e) {
        log.error(e.getMessage());
        return ResponseEntity
                .status(HttpStatus.NOT_FOUND)
                .body(Collections.singletonMap(messageKey, e.getMessage()));
    }

    @ExceptionHandler(MetadataCannotBeSavedException.class)
    public ResponseEntity<Map<String, Object>> handleException(MetadataCannotBeSavedException e) {
        log.error(e.getMessage());
        return ResponseEntity
                .status(HttpStatus.INTERNAL_SERVER_ERROR)
                .body(Collections.singletonMap(messageKey, e.getMessage()));
    }

    @ExceptionHandler(value = {MethodArgumentNotValidException.class})
    public ResponseEntity<Map<String, Object>> handleException(MethodArgumentNotValidException e) {
        log.error(e.getMessage());
        Map<String, Object> errors = new HashMap<>();
        e.getBindingResult().getAllErrors().forEach((error) -> {
            String fieldName = ((FieldError) error).getField();
            String errorMessage = error.getDefaultMessage();
            errors.put(fieldName, errorMessage);
        });
        return ResponseEntity.status(HttpStatus.BAD_REQUEST).body(errors);
    }
}
