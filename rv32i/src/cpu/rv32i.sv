module rv32i (
    input  logic        ck,
    input  logic        rstn,
    input  int          trace_fd,
    input  int          log_fd
);
    string exec_file;
    logic [31:0] mem [0:255];
    int bin_file;

    initial begin
        if (!$value$plusargs("EXEC=%s", exec_file)) exec_file = "program.bin";
        #1;

        bin_file = $fopen(exec_file, "rb");
        if (bin_file) begin
            $fread(mem, bin_file);
            $fclose(bin_file);
        end else begin
            $fdisplay(log_fd, "Error: Cannot open %s", exec_file);
        end

        for (int i = 0; i < 256; i++) begin
            if (mem[i] !== 32'bx) begin
                $fdisplay(trace_fd, "%08h: %08h", i * 4, mem[i]);
            end
        end
    end

    always_ff @(posedge ck or negedge rstn) begin
        if (!rstn) begin
            
        end else begin
            
        end
    end

endmodule