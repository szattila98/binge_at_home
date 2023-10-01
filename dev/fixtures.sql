INSERT INTO catalog (path, display_name, short_desc, long_desc)
VALUES
    ('test_catalog', 'Action Movies', 'Get your adrenaline pumping', 'Experience heart-pounding action and thrilling adventures with our collection of action-packed movies.'),
    ('test_catalog', 'Classic Cinema', 'Timeless films for film buffs', 'Explore the golden era of cinema with our curated selection of classic movies that have stood the test of time.'),
    ('test_catalog', 'Family-Friendly Flicks', 'Fun for the whole family', 'Enjoy quality time with your loved ones with our family-friendly movie collection, featuring wholesome entertainment for all ages.'),
    ('test_catalog', 'Sci-Fi and Fantasy', 'Journey to other worlds', 'Dive into the realm of science fiction and fantasy with mind-bending stories, futuristic technology, and mythical creatures.'),
    ('test_catalog', 'Romantic Movies', 'Love stories to melt your heart', 'Indulge in tales of love and romance that will warm your heart and leave you with all the feels.');

INSERT INTO metadata (size, duration, bitrate, width, height, framerate)
VALUES
    (901185, 30, '2000', '1280', '720', '30.0');

INSERT INTO video (path, display_name, short_desc, long_desc, catalog_id, sequent_id, metadata_id)
VALUES
    ('test_catalog/test_folder/test_video.webm', 'The Fast and the Furious', 'High-octane car chases', 'Buckle up for an adrenaline-fueled ride with fast cars and intense action in this iconic film series.', 1, NULL, 1),
    ('test_catalog/outer_test_folder/test_video.webm', 'Casablanca', 'A timeless romance', 'Experience the enduring love story set against the backdrop of World War II in the classic film "Casablanca."', 1, NULL, 1),
    ('test_catalog/test_folder/another_test_folder/another_test_video.webm', 'Toy Story', 'Toys come to life', 'Join Woody, Buzz Lightyear, and their toy friends in a heartwarming adventure filled with humor and imagination in "Toy Story."', 1, NULL, NULL),
    ('test_catalog/test_folder/test_video.webm', 'The Lord of the Rings', 'Epic fantasy saga', 'Embark on a grand journey through Middle-earth with hobbits, wizards, and epic battles in "The Lord of the Rings" trilogy.', 2, NULL, NULL),
    ('test_catalog/test_folder/test_video.webm', 'La La Land', 'A musical love story', 'Fall in love with the enchanting music and dance of "La La Land" as two aspiring artists pursue their dreams in Los Angeles.', 3, NULL, NULL);
