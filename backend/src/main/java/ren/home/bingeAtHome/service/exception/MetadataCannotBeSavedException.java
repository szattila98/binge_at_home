package ren.home.bingeAtHome.service.exception;

/**
 * Thrown when a Metadata cannot be saved.
 *
 * @author Attila Szőke
 */
public class MetadataCannotBeSavedException extends Exception {

    private static final String ERROR_MSG = "There was an error when trying to save the metadata!";

    public MetadataCannotBeSavedException() {
    }

    public MetadataCannotBeSavedException(String message) {
        super(message);
    }

    public MetadataCannotBeSavedException(String message, Throwable cause) {
        super(message, cause);
    }

    public MetadataCannotBeSavedException(Throwable cause) {
        super(cause);
    }

    public MetadataCannotBeSavedException(String message, Throwable cause, boolean enableSuppression, boolean writableStackTrace) {
        super(message, cause, enableSuppression, writableStackTrace);
    }
}
