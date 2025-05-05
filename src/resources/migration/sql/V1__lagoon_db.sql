
CREATE TABLE Reservation (
	ref_no UUID PRIMARY KEY,
	hotel_name TEXT NOT NULL,
	no_of_rooms SMALLINT NOT NULL,
	check_in_date DATE NOT NULL,
	check_out_date DATE NOT NULL,
	customer_name TEXT NOT NULL,
	reservation_time TIMESTAMP NOT NULL,
	total_amount REAL NOT NULL
);
