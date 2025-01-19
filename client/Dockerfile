FROM oven/bun

WORKDIR /app

# Copy package files first for better caching
COPY package*.json ./
COPY bun.lockb ./

# Install dependencies
RUN bun install

# Copy source code
COPY . .

# Expose port
EXPOSE 5173

# Set development environment
ENV NODE_ENV=development

# Start development server
CMD ["bun", "run", "dev", "--host"]
