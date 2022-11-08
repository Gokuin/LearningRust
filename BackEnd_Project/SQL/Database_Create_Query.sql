--Create table setup Query

	--Create the customers table
CREATE TABLE IF NOT EXISTS Customers  
(
	custID serial PRIMARY KEY,
	first_Name VARCHAR (50) NOT NULL,
	last_Name VARCHAR (50) NOT NULL,
	cust_email VARCHAR(250) NOT NULL,
	address VARCHAR(50) NOT NULL
);

	--Create the Products table
CREATE TABLE IF NOT EXISTS Products  
(
	prodID serial PRIMARY KEY,
	prod_Name VARCHAR (250) NOT NULL,
	price float8 NOT NULL,
	quantity_left int NOT NULL,
	prod_manufactor VARCHAR (250) NOT NULL
);

----Create the Orders table
CREATE TABLE IF NOT EXISTS Orders  
(
	orderID serial PRIMARY KEY,
	prodID int NOT NULL,
	custID int NOT NULL,
	order_date date NOT NULL,
	order_total float8 NOT NULL,
	--setup the foreign key realtionships
	FOREIGN KEY (prodID)
		REFERENCES Products (prodID),
	FOREIGN KEY (custID)
		REFERENCES Customers (custID)
);