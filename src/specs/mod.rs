/**
 * This file is required for writing tests that need Database connection, I can't for example
 * just place tests in entity workspace because I don't have access to test.rs file which requires migration
 * workspace, which requires entity workspace and this causes infinity loop
 */
mod entity;
