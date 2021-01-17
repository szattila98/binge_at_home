package ren.home.bingeAtHome.service.exception;

/**
 * Thrown when and image cannot be found.
 *
 * @author Attila Szőke
 */
public class ImageMissingException extends Exception {

    private static final String ERROR_MSG = "Queried image is missing from the image store!";

    public ImageMissingException() {
        super(ERROR_MSG);
    }

    public ImageMissingException(String message) {
        super(message);
    }

    public ImageMissingException(String message, Throwable cause) {
        super(message, cause);
    }

    public ImageMissingException(Throwable cause) {
        super(cause);
    }

    public ImageMissingException(String message, Throwable cause, boolean enableSuppression, boolean writableStackTrace) {
        super(message, cause, enableSuppression, writableStackTrace);
    }
}
