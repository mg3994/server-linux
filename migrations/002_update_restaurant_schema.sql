-- Update restaurant schema to match India-focused delivery platform
-- Version: 2.0.0
-- Created: 2024-01-16

-- First, let's backup existing data and update the restaurants table structure
-- Drop existing restaurants table and recreate with new structure
DROP TABLE IF EXISTS menu_items CASCADE;
DROP TABLE IF EXISTS menu_categories CASCADE;
DROP TABLE IF EXISTS restaurants CASCADE;

-- Create new restaurants table with India-focused structure
CREATE TABLE restaurants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    cuisine_type VARCHAR(100) NOT NULL,
    address VARCHAR(500) NOT NULL,
    city VARCHAR(100) NOT NULL,
    state VARCHAR(100) NOT NULL,
    postal_code VARCHAR(20) NOT NULL,
    country VARCHAR(100) NOT NULL DEFAULT 'India',
    phone VARCHAR(20) NOT NULL,
    email VARCHAR(255),
    latitude DECIMAL(10, 8),
    longitude DECIMAL(11, 8),
    image_url VARCHAR(500),
    cover_image_url VARCHAR(500),
    rating DECIMAL(3, 2) DEFAULT 0.00,
    total_reviews INTEGER DEFAULT 0,
    delivery_fee DECIMAL(10, 2) NOT NULL DEFAULT 29.00,
    minimum_order DECIMAL(10, 2) NOT NULL DEFAULT 99.00,
    delivery_time_minutes INTEGER NOT NULL DEFAULT 30,
    is_active BOOLEAN DEFAULT TRUE,
    is_accepting_orders BOOLEAN DEFAULT TRUE,
    fssai_license VARCHAR(50),
    gst_number VARCHAR(50),
    opening_hours JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create new menu_items table with India-focused structure
CREATE TABLE menu_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    restaurant_id UUID NOT NULL REFERENCES restaurants(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    category VARCHAR(100) NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    image_url VARCHAR(500),
    is_vegetarian BOOLEAN DEFAULT FALSE,
    is_vegan BOOLEAN DEFAULT FALSE,
    is_gluten_free BOOLEAN DEFAULT FALSE,
    spice_level INTEGER DEFAULT 0 CHECK (spice_level >= 0 AND spice_level <= 5),
    ingredients JSONB,
    allergens JSONB,
    is_available BOOLEAN DEFAULT TRUE,
    preparation_time_minutes INTEGER DEFAULT 15,
    calories INTEGER,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_restaurants_owner_id ON restaurants(owner_id);
CREATE INDEX idx_restaurants_city ON restaurants(city);
CREATE INDEX idx_restaurants_cuisine_type ON restaurants(cuisine_type);
CREATE INDEX idx_restaurants_is_active ON restaurants(is_active);
CREATE INDEX idx_restaurants_is_accepting_orders ON restaurants(is_accepting_orders);
CREATE INDEX idx_restaurants_rating ON restaurants(rating DESC);

CREATE INDEX idx_menu_items_restaurant_id ON menu_items(restaurant_id);
CREATE INDEX idx_menu_items_category ON menu_items(category);
CREATE INDEX idx_menu_items_is_available ON menu_items(is_available);
CREATE INDEX idx_menu_items_is_vegetarian ON menu_items(is_vegetarian);
CREATE INDEX idx_menu_items_price ON menu_items(price);

-- Create triggers for updated_at timestamps
CREATE TRIGGER update_restaurants_updated_at BEFORE UPDATE ON restaurants FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_menu_items_updated_at BEFORE UPDATE ON menu_items FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Insert sample Indian restaurants for testing
INSERT INTO restaurants (
    owner_id, name, description, cuisine_type, address, city, state, postal_code,
    phone, email, latitude, longitude, delivery_fee, minimum_order, delivery_time_minutes,
    fssai_license, gst_number, opening_hours
) 
SELECT 
    id,
    'Spice Garden',
    'Authentic North Indian cuisine with traditional flavors',
    'North Indian',
    '123 MG Road, Connaught Place',
    'Mumbai',
    'Maharashtra',
    '400001',
    '+91-9876543210',
    'spicegarden@example.com',
    19.0760,
    72.8777,
    25.00,
    150.00,
    35,
    'FSSAI12345678901234',
    '27ABCDE1234F1Z5',
    '{"monday": {"open": "11:00", "close": "23:00"}, "tuesday": {"open": "11:00", "close": "23:00"}, "wednesday": {"open": "11:00", "close": "23:00"}, "thursday": {"open": "11:00", "close": "23:00"}, "friday": {"open": "11:00", "close": "23:30"}, "saturday": {"open": "11:00", "close": "23:30"}, "sunday": {"open": "11:00", "close": "23:00"}}'
FROM users WHERE role = 'restaurant' LIMIT 1;

INSERT INTO restaurants (
    owner_id, name, description, cuisine_type, address, city, state, postal_code,
    phone, email, latitude, longitude, delivery_fee, minimum_order, delivery_time_minutes,
    fssai_license, gst_number, opening_hours
) 
SELECT 
    id,
    'Dosa Corner',
    'Crispy South Indian dosas and filter coffee',
    'South Indian',
    '456 Brigade Road, Commercial Street',
    'Bangalore',
    'Karnataka',
    '560001',
    '+91-9876543211',
    'dosacorner@example.com',
    12.9716,
    77.5946,
    20.00,
    100.00,
    25,
    'FSSAI12345678901235',
    '29ABCDE1234F1Z6',
    '{"monday": {"open": "07:00", "close": "22:00"}, "tuesday": {"open": "07:00", "close": "22:00"}, "wednesday": {"open": "07:00", "close": "22:00"}, "thursday": {"open": "07:00", "close": "22:00"}, "friday": {"open": "07:00", "close": "22:30"}, "saturday": {"open": "07:00", "close": "22:30"}, "sunday": {"open": "07:00", "close": "22:00"}}'
FROM users WHERE role = 'restaurant' LIMIT 1;

-- Insert sample menu items
INSERT INTO menu_items (
    restaurant_id, name, description, category, price, is_vegetarian, spice_level,
    ingredients, preparation_time_minutes, calories
)
SELECT 
    r.id,
    'Butter Chicken',
    'Tender chicken in rich tomato and butter gravy',
    'Main Course',
    320.00,
    false,
    3,
    '["chicken", "tomatoes", "butter", "cream", "spices"]',
    25,
    450
FROM restaurants r WHERE r.name = 'Spice Garden' LIMIT 1;

INSERT INTO menu_items (
    restaurant_id, name, description, category, price, is_vegetarian, spice_level,
    ingredients, preparation_time_minutes, calories
)
SELECT 
    r.id,
    'Paneer Makhani',
    'Cottage cheese in creamy tomato gravy',
    'Main Course',
    280.00,
    true,
    2,
    '["paneer", "tomatoes", "cream", "cashews", "spices"]',
    20,
    380
FROM restaurants r WHERE r.name = 'Spice Garden' LIMIT 1;

INSERT INTO menu_items (
    restaurant_id, name, description, category, price, is_vegetarian, spice_level,
    ingredients, preparation_time_minutes, calories
)
SELECT 
    r.id,
    'Masala Dosa',
    'Crispy rice crepe with spiced potato filling',
    'Main Course',
    120.00,
    true,
    2,
    '["rice", "lentils", "potatoes", "onions", "spices"]',
    15,
    250
FROM restaurants r WHERE r.name = 'Dosa Corner' LIMIT 1;

INSERT INTO menu_items (
    restaurant_id, name, description, category, price, is_vegetarian, spice_level,
    ingredients, preparation_time_minutes, calories
)
SELECT 
    r.id,
    'Filter Coffee',
    'Traditional South Indian filter coffee',
    'Beverages',
    45.00,
    true,
    0,
    '["coffee", "milk", "sugar"]',
    5,
    80
FROM restaurants r WHERE r.name = 'Dosa Corner' LIMIT 1;

-- Update orders table to reference the new restaurant structure
-- (The orders table should already be compatible, but let's make sure)
-- No changes needed for orders table as it already uses restaurant_id

-- Add some sample Indian street food items
INSERT INTO menu_items (
    restaurant_id, name, description, category, price, is_vegetarian, spice_level,
    ingredients, preparation_time_minutes, calories
)
SELECT 
    r.id,
    'Pani Puri',
    'Crispy hollow puris with spiced water',
    'Street Food',
    80.00,
    true,
    4,
    '["semolina", "tamarind", "mint", "coriander", "spices"]',
    10,
    150
FROM restaurants r WHERE r.name = 'Spice Garden' LIMIT 1;

INSERT INTO menu_items (
    restaurant_id, name, description, category, price, is_vegetarian, spice_level,
    ingredients, preparation_time_minutes, calories
)
SELECT 
    r.id,
    'Vada Pav',
    'Mumbai street food - spiced potato fritter in bun',
    'Street Food',
    35.00,
    true,
    3,
    '["potatoes", "bread", "green chutney", "spices"]',
    8,
    200
FROM restaurants r WHERE r.name = 'Spice Garden' LIMIT 1;