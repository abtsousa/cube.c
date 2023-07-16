#include <math.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

// Global variables
float A, B, C;  // Rotation angles

float cubeWidth = 20;  // Size of the cubes
int width = 160, height = 44;  // Dimensions of the screen
float zBuffer[160 * 44];  // Z-buffer for depth information
char buffer[160 * 44];  // Character buffer for ASCII representation
int backgroundASCIICode = '.';  // ASCII code for the background character
int distanceFromCam = 100;  // Distance of cubes from the camera
float horizontalOffset;  // Horizontal offset for cubes
float K1 = 40;  // Constant for perspective projection

float incrementSpeed = 0.6;  // Increment speed for cube generation

float x, y, z;  // Coordinates of a point on a cube's surface
float ooz;  // Inverse of z for perspective projection
int xp, yp;  // Screen coordinates of a point
int idx;  // Index in the buffer

// Function to calculate x-coordinate of a point on the cube's surface
float calculateX(int i, int j, int k) {
  return j * sin(A) * sin(B) * cos(C) - k * cos(A) * sin(B) * cos(C) +
         j * cos(A) * sin(C) + k * sin(A) * sin(C) + i * cos(B) * cos(C);
}

// Function to calculate y-coordinate of a point on the cube's surface
float calculateY(int i, int j, int k) {
  return j * cos(A) * cos(C) + k * sin(A) * cos(C) -
         j * sin(A) * sin(B) * sin(C) + k * cos(A) * sin(B) * sin(C) -
         i * cos(B) * sin(C);
}

// Function to calculate z-coordinate of a point on the cube's surface
float calculateZ(int i, int j, int k) {
  return k * cos(A) * cos(B) - j * sin(A) * cos(B) + i * sin(B);
}

// Function to calculate and update the buffer for a point on the cube's surface
void calculateForSurface(float cubeX, float cubeY, float cubeZ, int ch) {
  x = calculateX(cubeX, cubeY, cubeZ);
  y = calculateY(cubeX, cubeY, cubeZ);
  z = calculateZ(cubeX, cubeY, cubeZ) + distanceFromCam;

  ooz = 1 / z;

  xp = (int)(width / 2 + horizontalOffset + K1 * ooz * x * 2);
  yp = (int)(height / 2 + K1 * ooz * y);

  idx = xp + yp * width;
  if (idx >= 0 && idx < width * height) {
    if (ooz > zBuffer[idx]) {
      zBuffer[idx] = ooz;
      buffer[idx] = ch;
    }
  }
}

int main() {
  printf("\x1b[2J");  // Clear the screen
  while (1) {
    memset(buffer, backgroundASCIICode, width * height);  // Clear the character buffer
    memset(zBuffer, 0, width * height * 4);  // Clear the z-buffer
    cubeWidth = 20;  // Set properties for the first cube
    horizontalOffset = -2 * cubeWidth;

    // Generate the first cube
    for (float cubeX = -cubeWidth; cubeX < cubeWidth; cubeX += incrementSpeed) {
      for (float cubeY = -cubeWidth; cubeY < cubeWidth; cubeY += incrementSpeed) {
        calculateForSurface(cubeX, cubeY, -cubeWidth, '@');
        calculateForSurface(cubeWidth, cubeY, cubeX, '$');
        calculateForSurface(-cubeWidth, cubeY, -cubeX, '~');
        calculateForSurface(-cubeX, cubeY, cubeWidth, '#');
        calculateForSurface(cubeX, -cubeWidth, -cubeY, ';');
        calculateForSurface(cubeX, cubeWidth, cubeY, '+');
      }
    }

    cubeWidth = 10;  // Set properties for the second cube
    horizontalOffset = 1 * cubeWidth;

    // Generate the second cube
    for (float cubeX = -cubeWidth; cubeX < cubeWidth; cubeX += incrementSpeed) {
      for (float cubeY = -cubeWidth; cubeY < cubeWidth; cubeY += incrementSpeed) {
        calculateForSurface(cubeX, cubeY, -cubeWidth, '@');
        calculateForSurface(cubeWidth, cubeY, cubeX, '$');
        calculateForSurface(-cubeWidth, cubeY, -cubeX, '~');
        calculateForSurface(-cubeX, cubeY, cubeWidth, '#');
        calculateForSurface(cubeX, -cubeWidth, -cubeY, ';');
        calculateForSurface(cubeX, cubeWidth, cubeY, '+');
      }
    }

    cubeWidth = 5;  // Set properties for the third cube
    horizontalOffset = 8 * cubeWidth;

    // Generate the third cube
    for (float cubeX = -cubeWidth; cubeX < cubeWidth; cubeX += incrementSpeed) {
      for (float cubeY = -cubeWidth; cubeY < cubeWidth; cubeY += incrementSpeed) {
        calculateForSurface(cubeX, cubeY, -cubeWidth, '@');
        calculateForSurface(cubeWidth, cubeY, cubeX, '$');
        calculateForSurface(-cubeWidth, cubeY, -cubeX, '~');
        calculateForSurface(-cubeX, cubeY, cubeWidth, '#');
        calculateForSurface(cubeX, -cubeWidth, -cubeY, ';');
        calculateForSurface(cubeX, cubeWidth, cubeY, '+');
      }
    }

    printf("\x1b[H");  // Move cursor to the top-left of the screen
    for (int k = 0; k < width * height; k++) {
      putchar(k % width ? buffer[k] : 10);  // Print the character buffer
    }

    A += 0.05;  // Update rotation angles
    B += 0.05;
    C += 0.01;

    usleep(8000 * 2);  // Sleep for a short period of time
  }

  return 0;
}
