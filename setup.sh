#!/bin/bash
set -e

jar="bingeAtHome-0.1.0.jar"

echo "Building front-end!"
cd frontend
npm install
npm run build
cd ..
echo "Copying built front-end!"
cp -R frontend/dist/. backend/src/main/resources/static
echo "Building back-end!"
cd backend
mvn clean package
cd ..
echo "Copying built back-end!"
cp -R "backend/target/$jar" dist/
echo "Setup done!"