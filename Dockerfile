FROM nginx:alpine
ARG DEBIAN_FRONTEND=noninteractive

# Copy the build artifact from the build stage
COPY dist /usr/share/nginx/html

# Copy the nginx configuration
COPY ./nginx.conf /etc/nginx/conf.d/default.conf

# Expose the port
EXPOSE 8080

# Start Nginx server
CMD ["nginx", "-g", "daemon off;"]
